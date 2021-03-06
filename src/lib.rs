//! Raw bindings to the Electron API for projects using wasm-bindgen

#![deny(clippy::all)]
// #![deny(missing_docs)] // FIXME: wasm-bindgen macros break this

pub(crate) mod class;
pub(crate) mod interface;
pub(crate) mod module;

pub use class::*;
pub use interface::*;
pub use module::*;
