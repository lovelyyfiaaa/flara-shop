#[cfg(feature = "flathub-beta-http")]
pub mod flathub_beta;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "native")]
pub mod native;
pub mod prelude;
#[cfg(feature = "wasm32")]
pub use flathub_beta as auto;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "native")]
pub use native as auto;

pub mod schemas;
