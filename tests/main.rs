mod utils;

#[cfg(test)]
mod main {
    use crate::utils::{get_plugin_path, loader_init};
    use plux::Manager;
    use native_manager::NativePluginManager;

    #[test]
    fn load_manager() {
        loader_init();
    }

    #[test]
    fn load_plugin() {
        let mut loader = loader_init();

        loader
            .load_plugin_now(get_plugin_path("native_plugin", "1.0.0").to_str().unwrap())
            .unwrap();
    }

    #[test]
    fn read_symbol() {
        let mut loader = loader_init();

        loader
            .load_plugin_now(get_plugin_path("native_plugin", "1.0.0").to_str().unwrap())
            .unwrap();
        let plugin_manager = loader.get_manager_ref("npl").unwrap();
        // let manager =
    }
}
