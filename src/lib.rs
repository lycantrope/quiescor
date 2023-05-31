#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::Quiescor;


#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
pub use web::*;