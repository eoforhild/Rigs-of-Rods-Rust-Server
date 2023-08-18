pub mod config;
pub mod logger;
pub mod master_server;
pub mod listener;

use listener::Listener;
use tokio::sync::broadcast;
use config::{Config, ServerType, CONF};
use logger::LogLevel;
use master_server::{Client, retrieve_public_ip};

#[tokio::main]
async fn main() {
    // set default logger settings (haven't implemented yet)

    let conf: &Config = match Config::build() {
        // We know if true then CONF is not None so it's safe
        true => unsafe { &CONF.as_ref().unwrap() },
        false => {
            logger::log(LogLevel::Error, "Failed to parse server ini files, exiting...");
            return
        }
    };

    if conf.get_show_help() {
        conf.show_help();
        return;
    }

    if conf.get_show_version() {
        conf.show_version();
        return;
    }

    // Check configuration
    let server_mode: &ServerType = conf.get_server_mode();
    if let ServerType::Inet | ServerType::Auto = server_mode {
        logger::log(LogLevel::Info, "Starting server in INET mode");
        // Sets IP address automatically if it's not already set in ini files
        unsafe {
            let ip_addr: &str = conf.get_ip_addr();
            if ip_addr.is_empty() || ip_addr.eq("0.0.0.0") {
                logger::log(LogLevel::Warn, "No IP given, detecting...");
                if let Err(e) = retrieve_public_ip().await {
                    logger::log(LogLevel::Error, &e.to_string());
                    return
                };
            }
        }
        logger::log(LogLevel::Info, &format!("IP Address is: {}", conf.get_ip_addr()));

        // these may be outdated calculations for internet speed, probably not needed given
        // these were made when 100 mbps internet was less common and at most people had 10-25 mbps
        // and 1-2 mbps upload
        let max_clients: &u32 = conf.get_max_clients();
        logger::log(LogLevel::Info, &format!("Max required upload: {}kbit/s", max_clients * (max_clients - 1) * 64));
        logger::log(LogLevel::Info, &format!("Max required download: {}kbit/s", max_clients * 64));

        if conf.get_server_name().is_empty() {
            logger::log(LogLevel::Error, "Server name not specified, exiting...");
            return;
        }
        logger::log(LogLevel::Info, &format!("Server name: {}", &conf.get_server_name()));

        // master server registration should be in a seperate blocking thread
        let mut master: Client = Client::new();
        master.register();

        // start listener, blocking thread
        let listener = Listener::new(conf);
        let (sender, _) = broadcast::channel(16);
        if let Err(err) = listener.run(sender).await {
            eprintln!("Error occurred: {}", err);
            // Code to handle the error
        }
    }
}