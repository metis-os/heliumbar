// This will contain default informations to show in the bar

pub struct Info {
    pub os_release: String,
    pub uptime: String,
    pub kernel: String,
}

impl Info {
    pub fn new() -> Result<Info, Box<dyn std::error::Error>> {
        let os_release = nixinfo::distro().unwrap_or_else(|_| "N/A".to_string());
        let uptime = nixinfo::uptime().unwrap_or_else(|_| "N/A".to_string());
        let kernel = nixinfo::kernel().unwrap_or_else(|_| "N/A".to_string());

        Ok(Info {
            os_release,
            uptime,
            kernel,
        })
    }
}

