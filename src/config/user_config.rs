use crate::components::informations;
use crate::config::json_parser;
use directories::ProjectDirs;

pub fn read_config() -> informations::Info {
    if let Some(proj_dirs) = ProjectDirs::from("Heliumbar", "PwnWriter", "helium") {
        let config_dir = proj_dirs.config_dir();

        dbg!("{}", config_dir);

        let default_config = informations::Info::new().unwrap();
        let config_file = std::fs::read_to_string(config_dir.join("helium.conf"));

        println!("{}", config_file);
    }
}
