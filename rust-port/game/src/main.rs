mod components;

use components::{CameraController, CursorController};
use engine::scene::{Scene, TileRegistryEntry};
use engine::Game;
use game_desktop::DesktopPlatform;

fn main() {
    let width: usize = 800;
    let height: usize = 600;

    let mut game = Game::<DesktopPlatform>::new(width, height, "Explorer Game - Rust");

    // Register custom game components (PlayerController and WorldGenerator are now built-in)
    game.register_component("CameraController", |_value| {
        Box::new(CameraController::new())
    });

    game.register_component("CursorController", |_value| {
        Box::new(CursorController::new())
    });

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
                    "TileMapCollider": {},
                    "PlayerController": {}
                }
            },
            {
                "tag": "cursor",
                "components": {
                    "Transform": {
                        "position": { "x": 0, "y": 0, "z": 2 },
                        "scale": { "x": 1, "y": 1 }
                    },
                    "ColorRenderer": { "color": "#FFFFFF80" },
                    "CursorController": {}
                }
            },
            {
                "tag": "camera",
                "components": {
                    "Transform": {
                        "position": { "x": 0, "y": 0, "z": 0 }
                    },
                    "Camera": { "ppuX": 16, "ppuY": 16 },
                    "CameraController": {}
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
                    "LightMap": {},
                    "WorldGenerator": {}
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

    println!("Starting Explorer Game (Rust port)...");
    println!("Controls:");
    println!("  Arrow keys / WASD - Move");
    println!("  Space / Up / W - Jump");
    println!("  = / - : Zoom in/out");
    println!("  Left click - Place light tile");
    println!("  Right click - Erase tile");
    println!("  Escape - Quit");

    game.run();
}
