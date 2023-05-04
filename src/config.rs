use serde::Deserialize;


const DEFAULT_PATH: &str = "/etc/lyr/config.toml";

#[derive(Deserialize)]
pub struct Config {
    // pub shutdown_cmd: String,
    // pub reboot_cmd: String,
    // rounded: bool,
    pub show_fkeys: bool,
    // show_clock: bool,
    
    // user: String,
    // desktop: Desktop,
}

impl Config {
    pub fn new() -> Config { 
        let content = std::fs::read_to_string(DEFAULT_PATH).unwrap_or_else(|_|{
            eprintln!("Could not read config file at {}", DEFAULT_PATH);
            String::new()
        });

        toml::from_str::<_>(&content).unwrap_or_default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            // shutdown_cmd: String::from("shutdown -h now"),
            // reboot_cmd: String::from("reboot"),
            // rounded: true,
            show_fkeys: true,
            // show_clock: true,
            // user: String::from(""),
            // this shoudl not be a config it should be a cached thing
            // desktop: Desktop::default(),
        }
    }
}
