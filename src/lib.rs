#[cfg(feature = "plugin")] pub mod plugin;
#[cfg(feature = "binpkg")] pub mod binpkg;
mod constants;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");