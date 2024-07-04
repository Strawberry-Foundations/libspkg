pub trait Plugin {
    fn properties(&self) -> PluginProperties;
    fn execute(&self);
}

pub struct PluginProperties {
    pub name: &'static str,
    pub id: &'static str,
    pub package_id: &'static str,
    pub version: &'static str,
}
