# libspkg
Spkg Rust Library

# Use-case
libspkg can be used for Spkg plugins. This is also the main aspect of libspkg.
There is also binpkg, but this is currently only recommended for use with spkg

# Spkg Plugins 
```rust
use libspkg::plugin::{Plugin, PluginProperties};
pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
     fn execute(&self, args: &[String]) {
         match args.first().unwrap().as_str() {
             "test" => {
                 println!("Example spkg plugin")
             },
             _ => self.help()
         }
     }

     fn help(&self) {
         println!("Example help message")
     }
 }

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn create_plugin() -> (Box<dyn Plugin>, PluginProperties) {
    let properties: PluginProperties = PluginProperties {
        name: "Example Plugin",
        id: "example-plugin",
        package_id: "com.example.exampleplugin",
        version: env!("CARGO_PKG_VERSION"),
        library_version: libspkg::VERSION,
    };

    (Box::new(ExamplePlugin), properties)
}
```
