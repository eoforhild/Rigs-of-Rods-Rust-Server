pub mod config;
pub mod logger;
pub mod master_server;

use config::{Config, ServerType, CONF};
use logger::{LogLevel, LogType};

fn main() {
    // set default logger settings

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

    // Check configuration
    // let server_mode: &ServerType = conf.get_server_mode();
    // if let ServerType::Inet = server_mode {
    //     logger::log(LogLevel::Info, "Starting server in INET mode");
    //     let ip_addr: &str = conf.get_ip_addr();
    //     if ip_addr.is_empty() || ip_addr.eq("0.0.0.0") {
    //         logger::log(LogLevel::Warn, "No IP given, detecting")
    //         /* refer to the master server module to retrieve public IP */
    //         if (todo!()) {
    //             return;
    //         }
    //     }
    // }
}