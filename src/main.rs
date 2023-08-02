pub mod config;
pub mod logger;
pub mod master_server;

use config::{Config, ServerType, CONF};
use logger::{LogLevel, LogType};

fn main() {
    // Logger TODO!

    let conf: &Config = match Config::build() {
        // We know if true then CONF is not None so it's safe
        true => unsafe { &CONF.as_ref().unwrap() },
        false => {
            // Log down failure
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

    logger::log(LogLevel::Error, "asda");
    logger::log(LogLevel::Info, "asda");

    // Check configuration
    let server_mode: &ServerType = conf.get_server_mode();
    if let ServerType::Inet = server_mode {
        // Starting server in INET mode
        let ip_addr: &str = conf.get_ip_addr();
        if ip_addr.is_empty() || ip_addr.eq("0.0.0.0") {
            // No IP given, automatically detect IP
            /* refer to the master server module to retrieve public IP */
            if (todo!()) {
                // Fail condition, exit
            }
        }
    }
}