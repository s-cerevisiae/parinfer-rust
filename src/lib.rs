mod parinfer;
mod types;
mod changes;

// Native-specific stuff

#[cfg(not(target_arch = "wasm32"))]
mod c_wrapper;

#[cfg(not(target_arch = "wasm32"))]
pub use c_wrapper::run_parinfer;

#[cfg(not(target_arch = "wasm32"))]
pub use c_wrapper::INITIALIZED;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "emacs")]
mod emacs_wrapper;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "emacs")]
pub use emacs_wrapper::init;

// WebAssembly-specific stuff

#[cfg(target_arch = "wasm32")]
use stdweb::js_export;

mod common_wrapper;

#[cfg(target_arch = "wasm32")]
mod wasm_wrapper;

#[cfg(target_arch = "wasm32")]
#[js_export]
pub fn run_parinfer(input: String) -> String {
    wasm_wrapper::run_parinfer(input)
}
