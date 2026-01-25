use crate::component::{
    Component, InputContext, InputProvider, RenderContext, SetupContext, UpdateContext,
};
use crate::components::{
    Camera, ColorRenderer, LightMap, Physics, PlayerController, TileMap, TileMapCollider,
    TileRegistry, WorldGenerator,
};
use crate::entity::{Entity, EntityId};
use crate::frame_timer::FrameTimer;
use crate::input::{InputEvent, InputState, KeyCode, MouseButton};
use crate::platform::Platform;
use crate::renderer::{rgb, Renderer};
use crate::scene::{Scene, TileRegistryEntry};
use crate::types::{Vec2, Vec3};
use std::collections::HashMap;

/// Component factory function type
pub type ComponentFactory = Box<dyn Fn(&serde_json::Value) -> Box<dyn Component> + Send + Sync>;

/// Input provider implementation for the game
struct GameInputProvider<'a> {
    input_state: &'a InputState,
}

impl<'a> InputProvider for GameInputProvider<'a> {
    fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.input_state.is_key_pressed(key)
    }

    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.input_state.is_mouse_button_pressed(button)
    }

    fn mouse_position(&self) -> Vec2 {
        self.input_state.mouse_position()
    }
}

pub struct Game<P: Platform> {
    platform: P,
    renderer: Renderer,
    input_state: InputState,
    frame_timer: FrameTimer,

    entities: Vec<Entity>,
    next_entity_id: usize,
    camera_entity_id: Option<EntityId>,

    debug_lines: Vec<String>,
    component_factories: HashMap<String, ComponentFactory>,
}

impl<P: Platform> Game<P> {
    pub fn new(width: usize, height: usize, title: &str) -> Self {
        let platform = P::new(width, height, title);
        let renderer = Renderer::new(width, height);

        let mut game = Self {
            platform,
            renderer,
            input_state: InputState::new(),
            frame_timer: FrameTimer::new(),

            entities: Vec::new(),
            next_entity_id: 0,
            camera_entity_id: None,

            debug_lines: Vec::new(),
            component_factories: HashMap::new(),
        };

        // Register built-in component factories
        game.register_builtin_components();

        game
    }

    fn register_builtin_components(&mut self) {
        // Camera
        self.register_component("Camera", |value| {
            let mut camera = Camera::default();
            if let Some(obj) = value.as_object() {
                if let Some(ppu_x) = obj.get("ppuX").and_then(|v| v.as_f64()) {
                    camera.ppu_x = ppu_x as f32;
                }
                if let Some(ppu_y) = obj.get("ppuY").and_then(|v| v.as_f64()) {
                    camera.ppu_y = ppu_y as f32;
                }
            }
            Box::new(camera)
        });

        // ColorRenderer
        self.register_component("ColorRenderer", |value| {
            let mut renderer = ColorRenderer::default();
            if let Some(obj) = value.as_object() {
                if let Some(color) = obj.get("color").and_then(|v| v.as_str()) {
                    renderer.color = parse_color(color);
                }
            }
            Box::new(renderer)
        });

        // Physics
        self.register_component("Physics", |_value| Box::new(Physics::default()));

        // TileMap
        self.register_component("TileMap", |_value| Box::new(TileMap::new()));

        // TileMapCollider
        self.register_component("TileMapCollider", |_value| Box::new(TileMapCollider::new()));

        // TileRegistry
        self.register_component("TileRegistry", |_value| Box::new(TileRegistry::new()));

        // LightMap
        self.register_component("LightMap", |_value| Box::new(LightMap::new()));

        // WorldGenerator
        self.register_component("WorldGenerator", |value| {
            let seed = value
                .get("seed")
                .and_then(|v| v.as_u64())
                .unwrap_or(42) as u32;
            Box::new(WorldGenerator::new(seed))
        });

        // PlayerController
        self.register_component("PlayerController", |_value| {
            Box::new(PlayerController::new())
        });
    }

    pub fn register_component<F>(&mut self, name: &str, factory: F)
    where
        F: Fn(&serde_json::Value) -> Box<dyn Component> + Send + Sync + 'static,
    {
        self.component_factories
            .insert(name.to_string(), Box::new(factory));
    }

    pub fn load_scene(&mut self, scene: &Scene) {
        for scene_entity in &scene.entities {
            let entity_id = self.add_entity();

            if let Some(entity) = self.get_entity_mut(entity_id) {
                entity.tag = scene_entity.tag.clone();

                // Parse transform if present
                if let Some(transform_value) = scene_entity.components.get("Transform") {
                    if let Some(obj) = transform_value.as_object() {
                        if let Some(pos) = obj.get("position").and_then(|v| v.as_object()) {
                            entity.transform.position = Vec3::new(
                                pos.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                                pos.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                                pos.get("z").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                            );
                        }
                        if let Some(scale) = obj.get("scale").and_then(|v| v.as_object()) {
                            entity.transform.scale = Vec2::new(
                                scale.get("x").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32,
                                scale.get("y").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32,
                            );
                        }
                        if let Some(rotation) = obj.get("rotation").and_then(|v| v.as_f64()) {
                            entity.transform.rotation = rotation as f32;
                        }
                    }
                }
            }

            // Add components
            for (component_name, component_value) in &scene_entity.components {
                if component_name == "Transform" {
                    continue; // Already handled
                }

                if let Some(factory) = self.component_factories.get(component_name) {
                    let component = factory(component_value);
                    if let Some(entity) = self.get_entity_mut(entity_id) {
                        entity.add_component_boxed(component);
                    }
                }
            }

            // Set camera if this entity has a Camera component and tag is "camera"
            if let Some(entity) = self.get_entity(entity_id) {
                if entity.has_component::<Camera>()
                    && entity.tag.as_deref() == Some("camera")
                {
                    self.camera_entity_id = Some(entity_id);
                }
            }
        }

        // Setup all entities
        self.setup_entities();
    }

    pub fn register_tile_registry(&mut self, entries: Vec<TileRegistryEntry>) {
        for entity in &mut self.entities {
            if let Some(registry) = entity.get_component_mut::<TileRegistry>() {
                registry.register_tiles(entries.clone());
            }
        }
    }

    fn setup_entities(&mut self) {
        let width = self.renderer.width();
        let height = self.renderer.height();

        for entity in &mut self.entities {
            // Clone tag to avoid borrow conflict
            let tag = entity.tag.clone();
            let ctx = SetupContext {
                game_width: width,
                game_height: height,
                entity_tag: tag.as_deref(),
            };

            for component in entity.components_mut() {
                component.setup(&ctx);
            }
        }
    }

    pub fn add_entity(&mut self) -> EntityId {
        let id = EntityId(self.next_entity_id);
        self.next_entity_id += 1;
        self.entities.push(Entity::new(id));
        id
    }

    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

    pub fn get_entity_by_tag(&self, tag: &str) -> Option<EntityId> {
        self.entities
            .iter()
            .find(|e| e.tag.as_deref() == Some(tag))
            .map(|e| e.id)
    }

    pub fn debug(&mut self, msg: &str) {
        self.debug_lines.push(msg.to_string());
    }

    pub fn run(&mut self) -> ! {
        loop {
            let delta_time = self.frame_timer.record_frame();

            // Poll input events
            let events = self.platform.poll_events();
            for event in &events {
                self.input_state.process_event(event);
                self.dispatch_input_event(event);
            }

            // Update
            self.update(delta_time);

            // Render
            self.render();

            // Present
            if !self.platform.update(self.renderer.buffer()) {
                std::process::exit(0);
            }
        }
    }

    fn dispatch_input_event(&mut self, event: &InputEvent) {
        let input_provider = GameInputProvider {
            input_state: &self.input_state,
        };
        let ctx = InputContext {
            input: &input_provider,
        };

        for entity in &mut self.entities {
            if !entity.enabled {
                continue;
            }

            for component in entity.components_mut() {
                match event {
                    InputEvent::KeyDown(key) => component.on_key_down(&ctx, *key),
                    InputEvent::KeyUp(key) => component.on_key_up(&ctx, *key),
                    InputEvent::MouseDown(button, pos) => {
                        component.on_mouse_down(&ctx, *button, *pos)
                    }
                    InputEvent::MouseUp(button, pos) => component.on_mouse_up(&ctx, *button, *pos),
                    InputEvent::MouseMove(pos) => component.on_mouse_move(&ctx, *pos),
                }
            }
        }
    }

    fn update(&mut self, delta_time: f32) {
        let width = self.renderer.width();
        let height = self.renderer.height();

        let input_provider = GameInputProvider {
            input_state: &self.input_state,
        };

        let ctx = UpdateContext {
            delta_time,
            input: &input_provider,
            game_width: width,
            game_height: height,
        };

        // Update all entities
        for entity in &mut self.entities {
            if !entity.enabled {
                continue;
            }

            for component in entity.components_mut() {
                component.update(&ctx);
            }
        }

        // Cross-entity integration
        self.update_world_generator();
        self.update_camera_follow();
        self.update_cursor();
        self.update_player_input();

        // Handle physics and collisions
        self.update_physics(delta_time);

        // Update lighting
        self.update_lighting();

        // Remove destroyed entities
        self.entities.retain(|e| !e.marked_for_destruction);
    }

    fn update_world_generator(&mut self) {
        // Get camera bounds
        let camera_bounds = if let Some(camera_id) = self.camera_entity_id {
            if let Some(entity) = self.get_entity(camera_id) {
                if let Some(camera) = entity.get_component::<Camera>() {
                    Some(camera.get_bounds(&entity.transform.position, &entity.transform.scale))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        // Set bounds on WorldGenerator and collect pending tiles
        let mut pending_tiles: Vec<(i32, i32, u8)> = Vec::new();

        for entity in &mut self.entities {
            if let Some(world_gen) = entity.get_component_mut::<WorldGenerator>() {
                if let Some((min, max)) = camera_bounds {
                    world_gen.set_visible_bounds(min, max);
                }
                pending_tiles.extend(world_gen.take_pending_tiles());
            }
        }

        // Apply pending tiles to TileMap
        if !pending_tiles.is_empty() {
            for entity in &mut self.entities {
                if let Some(tile_map) = entity.get_component_mut::<TileMap>() {
                    for (x, y, tile_id) in &pending_tiles {
                        tile_map.set_tile(*x, *y, *tile_id);
                    }
                }
                // Also register light sources from light tiles
                if let Some(light_map) = entity.get_component_mut::<LightMap>() {
                    for (x, y, tile_id) in &pending_tiles {
                        if *tile_id == 4 {
                            // Light tile
                            light_map.add_light_source(*x, *y, 1.0, 10.0);
                        }
                    }
                }
            }
        }
    }

    fn update_camera_follow(&mut self) {
        // Get player position
        let player_pos = self
            .entities
            .iter()
            .find(|e| e.tag.as_deref() == Some("player"))
            .map(|e| e.transform.position);

        // Update camera position to follow player
        if let Some(player_pos) = player_pos {
            if let Some(camera_id) = self.camera_entity_id {
                if let Some(entity) = self.entities.iter_mut().find(|e| e.id == camera_id) {
                    // Smooth follow (lerp towards player)
                    let lerp_factor = 0.1;
                    entity.transform.position.x +=
                        (player_pos.x - entity.transform.position.x) * lerp_factor;
                    entity.transform.position.y +=
                        (player_pos.y - entity.transform.position.y) * lerp_factor;
                }
            }
        }
    }

    fn update_cursor(&mut self) {
        // Get camera info for screen-to-world conversion
        let (camera_pos, ppu) = if let Some(camera_id) = self.camera_entity_id {
            if let Some(entity) = self.get_entity(camera_id) {
                if let Some(camera) = entity.get_component::<Camera>() {
                    (entity.transform.position.xy(), camera.ppu())
                } else {
                    (Vec2::ZERO, Vec2::new(16.0, 16.0))
                }
            } else {
                (Vec2::ZERO, Vec2::new(16.0, 16.0))
            }
        } else {
            (Vec2::ZERO, Vec2::new(16.0, 16.0))
        };

        // Convert mouse screen position to world position
        let mouse_screen = self.input_state.mouse_position();
        let screen_center_x = self.renderer.width() as f32 / 2.0;
        let screen_center_y = self.renderer.height() as f32 / 2.0;

        let world_x = (mouse_screen.x - screen_center_x) / ppu.x + camera_pos.x;
        let world_y = (mouse_screen.y - screen_center_y) / ppu.y + camera_pos.y;

        // Update cursor entity position
        if let Some(cursor_entity) = self
            .entities
            .iter_mut()
            .find(|e| e.tag.as_deref() == Some("cursor"))
        {
            // Snap to tile grid
            cursor_entity.transform.position.x = world_x.floor();
            cursor_entity.transform.position.y = world_y.floor();
        }

        // Handle tile placement/erasure
        let tile_x = world_x.floor() as i32;
        let tile_y = world_y.floor() as i32;

        if self.input_state.is_mouse_button_pressed(MouseButton::Left) {
            // Place light tile
            for entity in &mut self.entities {
                if let Some(tile_map) = entity.get_component_mut::<TileMap>() {
                    if tile_map.get_tile(tile_x, tile_y) == 0 {
                        tile_map.set_tile(tile_x, tile_y, 4); // Light tile
                    }
                }
                if let Some(light_map) = entity.get_component_mut::<LightMap>() {
                    light_map.add_light_source(tile_x, tile_y, 1.0, 10.0);
                }
            }
        }

        if self.input_state.is_mouse_button_pressed(MouseButton::Right) {
            // Erase tile
            for entity in &mut self.entities {
                if let Some(tile_map) = entity.get_component_mut::<TileMap>() {
                    tile_map.set_tile(tile_x, tile_y, 0);
                }
                if let Some(light_map) = entity.get_component_mut::<LightMap>() {
                    light_map.remove_light_source(tile_x, tile_y);
                }
            }
        }
    }

    fn update_player_input(&mut self) {
        // Check if player is grounded
        let player_grounded = {
            let mut grounded = false;
            if let Some(player) = self
                .entities
                .iter()
                .find(|e| e.tag.as_deref() == Some("player"))
            {
                let pos = player.transform.position;
                let scale = player.transform.scale;
                let half_h = scale.y / 2.0;
                let check_y = pos.y + half_h + 0.1; // Check slightly below feet

                // Check tiles below player
                for entity in &self.entities {
                    if let Some(tile_map) = entity.get_component::<TileMap>() {
                        let tile_x_left = (pos.x - scale.x / 2.0).floor() as i32;
                        let tile_x_right = (pos.x + scale.x / 2.0).floor() as i32;
                        let tile_y = check_y.floor() as i32;

                        for tx in tile_x_left..=tile_x_right {
                            let tile_id = tile_map.get_tile(tx, tile_y);
                            if tile_id > 0 {
                                if let Some(registry) = entity.get_component::<TileRegistry>() {
                                    if registry.is_solid(tile_id) {
                                        grounded = true;
                                        break;
                                    }
                                } else {
                                    grounded = true;
                                    break;
                                }
                            }
                        }
                        if grounded {
                            break;
                        }
                    }
                }
            }
            grounded
        };

        // Sync PlayerController velocity to Physics and update grounded state
        for entity in &mut self.entities {
            if entity.tag.as_deref() == Some("player") {
                // Get velocity from PlayerController
                let player_velocity = entity
                    .get_component::<PlayerController>()
                    .map(|pc| pc.velocity);

                // Update grounded state
                if let Some(pc) = entity.get_component_mut::<PlayerController>() {
                    pc.grounded = player_grounded;
                }

                // Apply velocity to Physics (x from controller, keep y for gravity)
                if let Some(vel) = player_velocity {
                    if let Some(physics) = entity.get_component_mut::<Physics>() {
                        physics.velocity.x = vel.x;
                        // Only apply jump velocity if it's negative (jumping up)
                        if vel.y < 0.0 {
                            physics.velocity.y = vel.y;
                        }
                    }
                }
            }
        }
    }

    fn update_physics(&mut self, delta_time: f32) {
        // Collect physics entity data
        struct PhysicsData {
            entity_idx: usize,
            scale: Vec2,
        }

        let mut physics_entities: Vec<PhysicsData> = Vec::new();

        for (idx, entity) in self.entities.iter().enumerate() {
            if !entity.enabled {
                continue;
            }
            if entity.has_component::<Physics>() {
                physics_entities.push(PhysicsData {
                    entity_idx: idx,
                    scale: entity.transform.scale,
                });
            }
        }

        // Process each physics entity
        for phys_data in physics_entities {
            // Get entity transform
            let transform_clone = self.entities[phys_data.entity_idx].transform.clone();
            let scale = phys_data.scale;

            // Simulate physics with collision checking
            // We need to do this in a way that doesn't hold borrows across the closure
            let new_position = {
                let entity = &mut self.entities[phys_data.entity_idx];
                if let Some(physics) = entity.get_component_mut::<Physics>() {
                    physics.set_transform(&transform_clone);

                    // Simple physics step without complex collision (for now)
                    // Apply velocity
                    let mut vel = physics.velocity;
                    vel.x += physics.acceleration.x * delta_time;
                    vel.y += physics.acceleration.y * delta_time;

                    let mut new_x = transform_clone.position.x + vel.x * delta_time;
                    let mut new_y = transform_clone.position.y + vel.y * delta_time;

                    physics.velocity = vel;
                    Some((new_x, new_y, vel))
                } else {
                    None
                }
            };

            // Now check collisions and adjust position
            if let Some((mut new_x, mut new_y, mut vel)) = new_position {
                let half_w = scale.x / 2.0;
                let half_h = scale.y / 2.0;

                // Check X collision
                let check_x = |x: f32, y: f32| -> bool {
                    let corners = [
                        (x - half_w, y - half_h),
                        (x + half_w, y - half_h),
                        (x - half_w, y + half_h),
                        (x + half_w, y + half_h),
                    ];
                    for (cx, cy) in corners {
                        let tile_x = cx.floor() as i32;
                        let tile_y = cy.floor() as i32;
                        for entity in &self.entities {
                            if let Some(tile_map) = entity.get_component::<TileMap>() {
                                let tile_id = tile_map.get_tile(tile_x, tile_y);
                                if tile_id > 0 {
                                    if let Some(registry) = entity.get_component::<TileRegistry>() {
                                        if registry.is_solid(tile_id) {
                                            return true;
                                        }
                                    } else {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                    false
                };

                // X-axis collision
                if check_x(new_x, transform_clone.position.y) {
                    new_x = transform_clone.position.x;
                    vel.x = 0.0;
                }

                // Y-axis collision
                if check_x(new_x, new_y) {
                    new_y = transform_clone.position.y;
                    vel.y = 0.0;
                }

                // Update entity
                let entity = &mut self.entities[phys_data.entity_idx];
                entity.transform.position.x = new_x;
                entity.transform.position.y = new_y;
                if let Some(physics) = entity.get_component_mut::<Physics>() {
                    physics.velocity = vel;
                }
            }
        }
    }

    fn update_lighting(&mut self) {
        // Bake light maps
        for entity in &mut self.entities {
            if let Some(light_map) = entity.get_component_mut::<LightMap>() {
                light_map.bake();
            }
        }

        // Update tile map lighting data
        for entity in &mut self.entities {
            // Get light data first (immutable borrow)
            let light_data = entity
                .get_component::<LightMap>()
                .map(|lm| lm.get_all_lighting());

            // Then update tile map (mutable borrow) - this works because we use the result, not the reference
            if let Some(data) = light_data {
                if let Some(tile_map) = entity.get_component_mut::<TileMap>() {
                    tile_map.set_light_data(data);
                }
            }
        }
    }

    fn render(&mut self) {
        // Clear
        self.renderer.clear(rgb(20, 20, 30));

        // Get camera info
        let (camera_pos, camera_rotation, _camera_scale, camera_bounds, ppu) =
            if let Some(camera_id) = self.camera_entity_id {
                if let Some(entity) = self.get_entity(camera_id) {
                    if let Some(camera) = entity.get_component::<Camera>() {
                        let bounds =
                            camera.get_bounds(&entity.transform.position, &entity.transform.scale);
                        (
                            entity.transform.position.xy(),
                            entity.transform.rotation,
                            entity.transform.scale,
                            Some(bounds),
                            camera.ppu(),
                        )
                    } else {
                        (Vec2::ZERO, 0.0, Vec2::ONE, None, Vec2::new(16.0, 16.0))
                    }
                } else {
                    (Vec2::ZERO, 0.0, Vec2::ONE, None, Vec2::new(16.0, 16.0))
                }
            } else {
                (Vec2::ZERO, 0.0, Vec2::ONE, None, Vec2::new(16.0, 16.0))
            };

        // Render tile maps first
        if let Some(bounds) = camera_bounds {
            for entity in &mut self.entities {
                if !entity.enabled {
                    continue;
                }

                if let Some(tile_map) = entity.get_component_mut::<TileMap>() {
                    tile_map.render_visible_chunks(&mut self.renderer, bounds, ppu, camera_pos);
                }
            }
        }

        // Setup camera transform for entity rendering
        self.renderer.reset_transform();
        let screen_center_x = self.renderer.width() as f32 / 2.0;
        let screen_center_y = self.renderer.height() as f32 / 2.0;
        self.renderer.translate(screen_center_x, screen_center_y);
        self.renderer.scale(ppu.x, ppu.y);
        self.renderer.rotate(-camera_rotation);
        self.renderer.translate(-camera_pos.x, -camera_pos.y);

        // Sort entities by Z
        let mut render_order: Vec<usize> = (0..self.entities.len()).collect();
        render_order.sort_by(|&a, &b| {
            let za = self.entities[a].transform.position.z;
            let zb = self.entities[b].transform.position.z;
            za.partial_cmp(&zb).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Render entities
        for idx in render_order {
            let entity = &mut self.entities[idx];
            if !entity.enabled {
                continue;
            }

            // Update ColorRenderer transforms - clone transform first to avoid borrow conflict
            let transform_clone = entity.transform.clone();
            if let Some(cr) = entity.get_component_mut::<ColorRenderer>() {
                cr.set_transform(&transform_clone);
            }

            let mut ctx = RenderContext {
                renderer: &mut self.renderer,
                camera_bounds,
                ppu,
            };

            for component in entity.components() {
                component.render(&mut ctx);
            }
        }

        // Reset transform for UI
        self.renderer.reset_transform();

        // Render debug info
        let fps = self.frame_timer.get_fps();
        self.debug(&format!("FPS: {:.1}", fps));

        // Simple text rendering (just colored blocks for now - proper text would need a font)
        let mut y = 10;
        for line in &self.debug_lines {
            // Draw a small indicator for each debug line
            self.renderer.fill_rect_screen(10, y, 4, 4, rgb(255, 255, 255));
            y += 20;
            let _ = line; // In a full implementation, we'd render the text
        }

        self.debug_lines.clear();
    }

    // For WASM: initialize and render once without the blocking loop
    pub fn init_and_render(&mut self) {
        let delta_time = self.frame_timer.record_frame();

        // Poll input events
        let events = self.platform.poll_events();
        for event in &events {
            self.input_state.process_event(event);
            self.dispatch_input_event(event);
        }

        // Update
        self.update(delta_time);

        // Render
        self.render();

        // Present
        self.platform.update(self.renderer.buffer());
    }

    pub fn input(&self) -> &InputState {
        &self.input_state
    }

    pub fn width(&self) -> usize {
        self.renderer.width()
    }

    pub fn height(&self) -> usize {
        self.renderer.height()
    }
}

fn parse_color(color: &str) -> u32 {
    if color.starts_with('#') {
        let hex = &color[1..];
        if hex.len() == 6 {
            if let Ok(val) = u32::from_str_radix(hex, 16) {
                return 0xFF000000 | val;
            }
        } else if hex.len() == 8 {
            if let Ok(val) = u32::from_str_radix(hex, 16) {
                // RGBA to ARGB
                let r = (val >> 24) & 0xFF;
                let g = (val >> 16) & 0xFF;
                let b = (val >> 8) & 0xFF;
                let a = val & 0xFF;
                return (a << 24) | (r << 16) | (g << 8) | b;
            }
        }
    } else if color.starts_with("rgba(") {
        // Parse rgba(r, g, b, a)
        let inner = &color[5..color.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        if parts.len() == 4 {
            let r: u8 = parts[0].parse().unwrap_or(0);
            let g: u8 = parts[1].parse().unwrap_or(0);
            let b: u8 = parts[2].parse().unwrap_or(0);
            let a: f32 = parts[3].parse().unwrap_or(1.0);
            let a = (a * 255.0) as u8;
            return ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        }
    }
    0xFFFF0000 // Default red
}
