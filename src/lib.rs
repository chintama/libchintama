#[macro_use]
mod vector;

pub mod components;
pub mod entities;
pub mod events;
pub mod resources;

mod client;
mod server;

mod collide;
mod config;
mod error;
mod systems;

pub mod prelude {
    pub use crate::entities::CreateEntity;
}

pub use crate::config::{Config, ConfigBuilder};
pub use crate::error::Result;
pub use crate::systems::Systems;
pub use crate::vector::Vector;

pub use crate::client::screen::run_client;
pub use crate::server::run_server;
