use engine::input::{InputEvent, KeyCode, MouseButton};
use engine::types::Vec2;
use engine::{Game, Platform};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, ImageData, KeyboardEvent, MouseEvent,
    Window,
};

/// Shared input state that can be accessed from event callbacks
struct InputState {
    pending_events: Vec<InputEvent>,
    pressed_keys: HashSet<KeyCode>,
    pressed_mouse_buttons: HashSet<MouseButton>,
    mouse_position: Vec2,
}

impl InputState {
    fn new() -> Self {
        Self {
            pending_events: Vec::new(),
            pressed_keys: HashSet::new(),
            pressed_mouse_buttons: HashSet::new(),
            mouse_position: Vec2::ZERO,
        }
    }
}

pub struct WebPlatform {
    context: CanvasRenderingContext2d,
    width: usize,
    height: usize,
    input_state: Rc<RefCell<InputState>>,
    // Store closures to prevent them from being dropped
    _keydown_closure: Closure<dyn FnMut(KeyboardEvent)>,
    _keyup_closure: Closure<dyn FnMut(KeyboardEvent)>,
    _mousedown_closure: Closure<dyn FnMut(MouseEvent)>,
    _mouseup_closure: Closure<dyn FnMut(MouseEvent)>,
    _mousemove_closure: Closure<dyn FnMut(MouseEvent)>,
}

fn js_key_to_keycode(key: &str) -> Option<KeyCode> {
    match key {
        "ArrowUp" => Some(KeyCode::ArrowUp),
        "ArrowDown" => Some(KeyCode::ArrowDown),
        "ArrowLeft" => Some(KeyCode::ArrowLeft),
        "ArrowRight" => Some(KeyCode::ArrowRight),
        " " => Some(KeyCode::Space),
        "Enter" => Some(KeyCode::Enter),
        "Escape" => Some(KeyCode::Escape),
        "=" | "+" => Some(KeyCode::Equal),
        "-" | "_" => Some(KeyCode::Minus),
        "w" | "W" => Some(KeyCode::KeyW),
        "a" | "A" => Some(KeyCode::KeyA),
        "s" | "S" => Some(KeyCode::KeyS),
        "d" | "D" => Some(KeyCode::KeyD),
        "0" => Some(KeyCode::Digit0),
        "1" => Some(KeyCode::Digit1),
        "2" => Some(KeyCode::Digit2),
        "3" => Some(KeyCode::Digit3),
        "4" => Some(KeyCode::Digit4),
        "5" => Some(KeyCode::Digit5),
        "6" => Some(KeyCode::Digit6),
        "7" => Some(KeyCode::Digit7),
        "8" => Some(KeyCode::Digit8),
        "9" => Some(KeyCode::Digit9),
        _ => None,
    }
}

fn js_button_to_mouse_button(button: i16) -> Option<MouseButton> {
    match button {
        0 => Some(MouseButton::Left),
        1 => Some(MouseButton::Middle),
        2 => Some(MouseButton::Right),
        _ => None,
    }
}

impl WebPlatform {
    fn setup_event_listeners(
        document: &Document,
        canvas: &HtmlCanvasElement,
        input_state: Rc<RefCell<InputState>>,
    ) -> (
        Closure<dyn FnMut(KeyboardEvent)>,
        Closure<dyn FnMut(KeyboardEvent)>,
        Closure<dyn FnMut(MouseEvent)>,
        Closure<dyn FnMut(MouseEvent)>,
        Closure<dyn FnMut(MouseEvent)>,
    ) {
        // Keydown listener
        let input_state_keydown = Rc::clone(&input_state);
        let keydown_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            if let Some(keycode) = js_key_to_keycode(&event.key()) {
                let mut state = input_state_keydown.borrow_mut();
                if !state.pressed_keys.contains(&keycode) {
                    state.pressed_keys.insert(keycode);
                    state.pending_events.push(InputEvent::KeyDown(keycode));
                }
                // Prevent default for arrow keys and space to avoid scrolling
                if matches!(
                    keycode,
                    KeyCode::ArrowUp
                        | KeyCode::ArrowDown
                        | KeyCode::ArrowLeft
                        | KeyCode::ArrowRight
                        | KeyCode::Space
                ) {
                    event.prevent_default();
                }
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        // Keyup listener
        let input_state_keyup = Rc::clone(&input_state);
        let keyup_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            if let Some(keycode) = js_key_to_keycode(&event.key()) {
                let mut state = input_state_keyup.borrow_mut();
                state.pressed_keys.remove(&keycode);
                state.pending_events.push(InputEvent::KeyUp(keycode));
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        // Mousedown listener
        let input_state_mousedown = Rc::clone(&input_state);
        let mousedown_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            if let Some(button) = js_button_to_mouse_button(event.button()) {
                let mut state = input_state_mousedown.borrow_mut();
                state.pressed_mouse_buttons.insert(button);
                let pos = Vec2::new(event.offset_x() as f32, event.offset_y() as f32);
                state.mouse_position = pos;
                state.pending_events.push(InputEvent::MouseDown(button, pos));
            }
            event.prevent_default();
        }) as Box<dyn FnMut(MouseEvent)>);

        // Mouseup listener
        let input_state_mouseup = Rc::clone(&input_state);
        let mouseup_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            if let Some(button) = js_button_to_mouse_button(event.button()) {
                let mut state = input_state_mouseup.borrow_mut();
                state.pressed_mouse_buttons.remove(&button);
                let pos = Vec2::new(event.offset_x() as f32, event.offset_y() as f32);
                state.mouse_position = pos;
                state.pending_events.push(InputEvent::MouseUp(button, pos));
            }
        }) as Box<dyn FnMut(MouseEvent)>);

        // Mousemove listener
        let input_state_mousemove = Rc::clone(&input_state);
        let mousemove_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            let mut state = input_state_mousemove.borrow_mut();
            let pos = Vec2::new(event.offset_x() as f32, event.offset_y() as f32);
            state.mouse_position = pos;
            state.pending_events.push(InputEvent::MouseMove(pos));
        }) as Box<dyn FnMut(MouseEvent)>);

        // Add event listeners to document for keyboard
        document
            .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())
            .expect("Failed to add keydown listener");
        document
            .add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())
            .expect("Failed to add keyup listener");

        // Add event listeners to canvas for mouse
        canvas
            .add_event_listener_with_callback(
                "mousedown",
                mousedown_closure.as_ref().unchecked_ref(),
            )
            .expect("Failed to add mousedown listener");
        canvas
            .add_event_listener_with_callback("mouseup", mouseup_closure.as_ref().unchecked_ref())
            .expect("Failed to add mouseup listener");
        canvas
            .add_event_listener_with_callback(
                "mousemove",
                mousemove_closure.as_ref().unchecked_ref(),
            )
            .expect("Failed to add mousemove listener");

        // Prevent context menu on right click
        let contextmenu_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            event.prevent_default();
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas
            .add_event_listener_with_callback(
                "contextmenu",
                contextmenu_closure.as_ref().unchecked_ref(),
            )
            .expect("Failed to add contextmenu listener");
        contextmenu_closure.forget(); // We don't need to keep this one

        (
            keydown_closure,
            keyup_closure,
            mousedown_closure,
            mouseup_closure,
            mousemove_closure,
        )
    }
}

impl Platform for WebPlatform {
    fn new(width: usize, height: usize, _title: &str) -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        // Get canvas with id "viewport"
        let canvas = document
            .get_element_by_id("viewport")
            .expect("should find canvas with id 'viewport'")
            .dyn_into::<HtmlCanvasElement>()
            .expect("element should be HtmlCanvasElement");

        // Set canvas size
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        // Make canvas focusable for keyboard events
        canvas.set_tab_index(0);
        let _ = canvas.focus();

        let context = canvas
            .get_context("2d")
            .expect("should get 2d context")
            .expect("should unwrap 2d context")
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("should cast to CanvasRenderingContext2d");

        let input_state = Rc::new(RefCell::new(InputState::new()));

        let (
            keydown_closure,
            keyup_closure,
            mousedown_closure,
            mouseup_closure,
            mousemove_closure,
        ) = Self::setup_event_listeners(&document, &canvas, Rc::clone(&input_state));

        Self {
            context,
            width,
            height,
            input_state,
            _keydown_closure: keydown_closure,
            _keyup_closure: keyup_closure,
            _mousedown_closure: mousedown_closure,
            _mouseup_closure: mouseup_closure,
            _mousemove_closure: mousemove_closure,
        }
    }

    fn update(&mut self, buffer: &[u32]) -> bool {
        // Convert u32 ARGB buffer to RGBA bytes for ImageData
        let mut rgba_buffer: Vec<u8> = Vec::with_capacity(buffer.len() * 4);

        for &pixel in buffer {
            // Extract ARGB components
            let a = ((pixel >> 24) & 0xFF) as u8;
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;

            // Push as RGBA
            rgba_buffer.push(r);
            rgba_buffer.push(g);
            rgba_buffer.push(b);
            rgba_buffer.push(if a == 0 { 255 } else { a }); // Default to opaque if alpha is 0
        }

        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&rgba_buffer),
            self.width as u32,
            self.height as u32,
        )
        .expect("should create ImageData");

        self.context
            .put_image_data(&image_data, 0.0, 0.0)
            .expect("should put image data");

        // Return true to keep running (WASM apps don't typically close)
        true
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn poll_events(&mut self) -> Vec<InputEvent> {
        let mut state = self.input_state.borrow_mut();
        std::mem::take(&mut state.pending_events)
    }

    fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.input_state.borrow().pressed_keys.contains(&key)
    }

    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.input_state
            .borrow()
            .pressed_mouse_buttons
            .contains(&button)
    }

    fn mouse_position(&self) -> Vec2 {
        self.input_state.borrow().mouse_position
    }
}

// Store the game globally for the animation loop
thread_local! {
    static GAME: RefCell<Option<Game<WebPlatform>>> = RefCell::new(None);
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame`");
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    let width: usize = 800;
    let height: usize = 600;

    let game = Game::<WebPlatform>::new(width, height, "Explorer Game");

    // Store the game
    GAME.with(|g| {
        *g.borrow_mut() = Some(game);
    });

    // Load scene (we need to do this after storing)
    GAME.with(|g| {
        if let Some(game) = g.borrow_mut().as_mut() {
            load_game_data(game);
        }
    });

    // Start the animation loop
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = Rc::clone(&f);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        GAME.with(|game_cell| {
            if let Some(game) = game_cell.borrow_mut().as_mut() {
                game.init_and_render();
            }
        });

        // Schedule next frame
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    // Start the loop
    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn load_game_data(game: &mut Game<WebPlatform>) {
    use engine::scene::{Scene, TileRegistryEntry};

    // Register custom game components
    // Note: In a full implementation, these would be separate crates or modules
    // For now, we'll register them inline

    // Load scene
    let scene_json = r##"{
        "entities": [
            {
                "tag": "player",
                "components": {
                    "Transform": {
                        "position": { "x": 0, "y": -5, "z": 1 },
                        "scale": { "x": 0.8, "y": 1.8 }
                    },
                    "ColorRenderer": { "color": "#FF0000" },
                    "Physics": {},
                    "TileMapCollider": {}
                }
            },
            {
                "tag": "cursor",
                "components": {
                    "Transform": {
                        "position": { "x": 0, "y": 0, "z": 2 },
                        "scale": { "x": 1, "y": 1 }
                    },
                    "ColorRenderer": { "color": "#FFFFFF80" }
                }
            },
            {
                "tag": "camera",
                "components": {
                    "Transform": {
                        "position": { "x": 0, "y": 0, "z": 0 }
                    },
                    "Camera": { "ppuX": 16, "ppuY": 16 }
                }
            },
            {
                "tag": "editableTileMap",
                "components": {
                    "Transform": {
                        "position": { "x": 0, "y": 0, "z": 0 }
                    },
                    "TileMap": {},
                    "TileRegistry": {},
                    "LightMap": {}
                }
            }
        ]
    }"##;

    let scene = Scene::from_json(scene_json).expect("Failed to parse scene JSON");
    game.load_scene(&scene);

    // Load tile registry
    let tile_registry_json = r#"[
        {
            "tileId": 1,
            "name": "Dirt",
            "assetPath": "dirt.png",
            "solid": true
        },
        {
            "tileId": 2,
            "name": "Grass",
            "assetPath": "grass.png",
            "solid": true
        },
        {
            "tileId": 3,
            "name": "Stone",
            "assetPath": "stone.png",
            "solid": true
        },
        {
            "tileId": 4,
            "name": "Light",
            "assetPath": "light.png",
            "solid": false,
            "lightIntensity": 1.0,
            "lightRadius": 10
        }
    ]"#;

    let tile_entries: Vec<TileRegistryEntry> =
        serde_json::from_str(tile_registry_json).expect("Failed to parse tile registry JSON");
    game.register_tile_registry(tile_entries);
}
