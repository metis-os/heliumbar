pub struct UserConfig {
    pub os_info: String,
    pub kernel: String,
    pub uptime: String,
}

pub fn parse_user_config(config_file: &str) -> Result<UserConfig, E> {
    // Parse JSON into struct using serde_json
    let config: UserConfig = serde_json::from_str(&contents)?;

    Ok(config)
}



