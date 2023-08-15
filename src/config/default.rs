use super::{Config, ServerType};

impl Default for Config {
    fn default() -> Config {
        Config {
            s_server_name: String::from("rust server test"),
            s_terrain_name: String::from("any"),
            s_public_password: String::from("colonthree"),
            s_scriptname: Default::default(),
            s_authfile: String::from("server.auth"),
            s_motdfile: String::from("server.motd"),
            s_rulesfile: String::from("server.rules"),
            s_blacklistfile: String::from("server.blacklist"),
            s_owner: Default::default(),
            s_website: Default::default(),
            s_irc: Default::default(),
            s_voip: Default::default(),
            s_serverlist_host: String::from("https://api.rigsofrods.org"),
            s_serverlist_path: String::from("https://api.rigsofrods.org"),
            s_resourcedir: String::from(""),

            s_server_mode: ServerType::Inet,
            s_ip_addr: String::from("0.0.0.0"),
            s_listen_port: 0,
            s_max_clients: 16,
            s_heartbeat_retry_count: 5,
            s_heartbeat_retry_seconds: 15,
            s_heartbeat_interval_sec: 60,

            s_print_stats: false,
            s_foreground: false,
            s_show_version: false,
            s_show_help: false,

            s_max_vehicles: 20,
            s_spawn_interval_sec: 0,
            s_max_spawn_rate: 0,

            s_spamfilter_msg_interval_sec: 0,
            s_spamfilter_msg_count: 0,
            s_spamfilter_gag_duration_sec: 10,
        }
    }
}