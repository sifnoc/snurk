#![cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "wasm")]
pub use wasm::*;

#[cfg(feature = "build")]
pub mod circuit;

#[cfg(feature = "build")]
pub use circuit::*;