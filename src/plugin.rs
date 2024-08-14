/// # libspkg Plugin Feature
/// Plugins are extensions for spkg that add functionality,
/// such as sandbox or certain other things that improve spkg.
/// Plugins can be programmed by the open source community in Rust
/// ```
/// use libspkg::plugin::{Plugin, PluginProperties};
/// pub struct ExamplePlugin;
///
/// impl Plugin for ExamplePlugin {
///     fn execute(&self, args: &[String]) {
///         match args.first().unwrap().as_str() {
///             "test" => {
///                 println!("Example spkg plugin")
///             },
///             _ => self.help()
///         }
///     }
///
///     fn help(&self) {
///         println!("Example help message")
///     }
/// }
///
/// #[allow(improper_ctypes_definitions)]
/// #[no_mangle]
/// pub extern "C" fn create_plugin() -> (Box<dyn Plugin>, PluginProperties) {
///     let properties: PluginProperties = PluginProperties {
///         name: "Example Plugin",
///         id: "example-plugin",
///         package_id: "com.example.exampleplugin",
///         version: env!("CARGO_PKG_VERSION"),
///         library_version: libspkg::VERSION,
///     };
///
///     (Box::new(ExamplePlugin), properties)
/// }
/// ```

pub trait Plugin {
    fn execute(&self, args: &[String]);
    fn help(&self);
}

/// # Struct PluginProperties
/// This struct holds various information about the plugin, i.e. name, version, ...
#[derive(Copy, Debug, Clone)]
pub struct PluginProperties {
    pub name: &'static str,
    pub id: &'static str,
    pub package_id: &'static str,
    pub version: &'static str,
    pub library_version: &'static str,
}
