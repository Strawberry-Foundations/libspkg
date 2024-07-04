/// # libspkg Plugin Feature
/// Plugins are extensions for spkg that add functionality,
/// such as sandbox or certain other things that improve spkg.
/// Plugins can be programmed by the open source community in Rust
/// ```
/// use libspkg::plugin::{Plugin, PluginProperties};
/// pub struct ExamplePlugin;
///
/// impl Plugin for ExamplePlugin {
///     fn properties(&self) -> PluginProperties {
///         PluginProperties {
///             name: "Example Plugin",
///             id: "example-plugin",
///             package_id: "com.example.exampleplugin",
///             version: "0.1.0",
///         }
///     }
///
///     fn execute(&self, args: &[String]) {
///         match args.first().unwrap().as_str() {
///             "test" => {
///                 println!("Test function")
///             },
///             _ => self.help()
///         }
///         println!("Example spkg plugin")
///     }
///
///     fn help(&self) {
///         println!("Example help message")
///     }
/// }
/// ```
pub trait Plugin {
    fn properties(&self) -> PluginProperties;
    fn execute(&self, args: &[String]);
    fn help(&self);
}

/// # Struct PluginProperties
/// This struct holds various information about the plugin, i.e. name, version, ...
pub struct PluginProperties {
    pub name: &'static str,
    pub id: &'static str,
    pub package_id: &'static str,
    pub version: &'static str,
}
