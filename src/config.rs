// use clap::{App, Arg};
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub base_directory: String,
    pub port: u16,
    pub password_hash: String,
    pub styles: Vec<String>,
    pub scripts: Vec<String>,

    pub disable_sidebar: bool,
    pub hidden_regex: Vec<String>,
    pub hide_folders: bool,
    pub hide_parent_folders: bool,
    pub max_icon_size: usize,
    pub modes: Vec<String>,
    pub mode_toggle_enabled: bool,
    pub sizes: Vec<usize>,

    pub auto_refresh_enabled: bool,
    pub auto_refresh_interval: usize,

    pub breadcrumbs_enabled: bool,

    pub enable_download: bool,

    pub calculate_folder_size_recusively: bool,
}

pub fn get_config() -> &'static Config {
    static CFG: Lazy<Config> = Lazy::new(|| Config::new());

    &*CFG
}

impl Config {
    pub fn new() -> Self {
        // TODO Cli related task to make easy to start server without config file
        // let mut app = App::new("r5ai".to_owned())
        //     .version("1.0.0")
        //     .about("Modern Web Index")
        //     .arg(Arg::with_name("port").short("p").help("Port to listen"))
        //     .arg(
        //         Arg::with_name("html_path")
        //             .long("html_path")
        //             .required(true)
        //             .takes_value(true)
        //             .help("Set html path which shall be served as homepage"),
        //     )
        //     .get_matches();

        let config_file_bytes = include_str!("../config.toml");

        // TODO if there is error in config file we must handle it properly
        let config: Config = toml::from_str(config_file_bytes).expect("Invalid config file");
        config
    }
}
