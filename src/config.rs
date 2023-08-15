mod print;
mod default;

pub static RORNET_VERSION: &str = "RoRnet_2.44";
pub static mut CONF: Option<Config> = None;

pub enum ServerType {
    Lan,
    Inet,
    Auto,
}

pub struct Config {
    s_server_name: String,
    s_terrain_name: String,
    s_public_password: String,
    s_ip_addr: String,
    s_scriptname: String,
    s_authfile: String,
    s_motdfile: String,
    s_rulesfile: String,
    s_blacklistfile: String,
    s_owner: String,
    s_website: String,
    s_irc: String,
    s_voip: String,
    s_serverlist_host: String,
    s_serverlist_path: String,
    s_resourcedir: String,

    s_listen_port: u32,
    s_max_clients: u32,
    s_heartbeat_retry_count: u32,
    s_heartbeat_retry_seconds: u32,
    s_heartbeat_interval_sec: u32,

    s_print_stats: bool,
    s_foreground: bool,
    s_show_version: bool,
    s_show_help: bool,

    // Vehicle spawn limits
    s_max_vehicles: usize,
    s_spawn_interval_sec: i32,
    s_max_spawn_rate:i32,

    s_server_mode: ServerType,

    s_spamfilter_msg_interval_sec: i32,
    s_spamfilter_msg_count: i32,
    s_spamfilter_gag_duration_sec: i32,
}

impl Config {
    // Builds a config object for the server 
    // Need to add actual argument and ini file parsing but defaults will work for now.
    pub fn build() -> bool{
        unsafe {
            CONF = Some(Default::default());
        }
        true
    }

    pub fn get_show_help(&self) -> bool { self.s_show_help }
    pub fn get_show_version(&self) -> bool { self.s_show_version }
    pub fn get_server_mode(&self) -> &ServerType { &self.s_server_mode }
    pub fn get_ip_addr(&self) -> &str { &self.s_ip_addr }
    pub fn get_listen_port(&self) -> &u32 { &self.s_listen_port }
    pub fn get_server_name(&self) -> &str { &self.s_server_name }
    pub fn get_terrain_name(&self) -> &str { &self.s_terrain_name }
    pub fn get_max_clients(&self) -> &u32 { &self.s_max_clients }
    pub fn get_public_pw(&self) -> &str { &self.s_public_password }
    pub fn get_serverlist_path(&self) -> &str { &self.s_serverlist_path }
    pub fn get_serverlist_host(&self) -> &str { &self.s_serverlist_host }

    pub fn set_ip_addr(&mut self, ip: &str) {
        self.s_ip_addr = ip.to_string();
    }
    
    pub fn is_public(&self) -> bool { !&self.get_public_pw().is_empty() }

}