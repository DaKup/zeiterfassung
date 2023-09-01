#![warn(clippy::all)]

pub mod native;
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
pub use web::*;

#[allow(dead_code)]
pub fn spawn_async<F: std::future::Future<Output = ()> + 'static>(fut: F) {
    #[cfg(not(target_arch = "wasm32"))]
    pollster::block_on(fut);

    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(fut);
}
