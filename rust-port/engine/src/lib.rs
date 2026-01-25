pub mod component;
pub mod entity;
pub mod frame_timer;
pub mod game;
pub mod input;
pub mod platform;
pub mod renderer;
pub mod scene;
pub mod types;

pub mod components;

pub use component::{Component, ComponentContext};
pub use entity::{Entity, EntityId};
pub use frame_timer::FrameTimer;
pub use game::Game;
pub use input::{InputEvent, InputState, KeyCode, MouseButton};
pub use platform::Platform;
pub use renderer::Renderer;
pub use types::{decode_address, encode_address, Address, Transform, Vec2, Vec3};
