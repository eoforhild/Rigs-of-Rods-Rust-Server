static RORNET_VERSION: &str = "RoRnet_2.44";
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
            s_serverlist_host: String::from("api.rigsofrods.org"),
            s_serverlist_path: String::from("api.rigsofrods.org"),
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

impl Config {
    // Builds a config object for the server
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

    pub fn show_help(&self) {
        println!(                
        "Usage: rorserver [OPTIONS]\n
        [OPTIONS] can be in Un*x `--help` or windows `/help` notation\n
        \n
         -config-file (-c) <INI file> Loads the configuration from a file\n
         -name <name>                 Name of the server, no spaces, only\n
                                      [a-z,0-9,A-Z]\n
         -terrain <mapname>           Map name (defaults to 'any')\n
         -max-clients|speed <clients> Maximum clients allowed\n
         -lan|inet                    Private or public server (defaults to inet)\n
        \n
         -password <password>         Private server password\n
         -ip <ip>                     Public IP address to register with.\n
         -port <port>                 Port to use (defaults to random 12000-12500)\n
         -verbosity {{0-5}}             Sets displayed log verbosity\n
         -log-verbosity {{0-5}}         Sets file log verbositylog verbosity\n
                                      levels available to verbosity and logverbosity:\n
                                          0 = stack\n
                                          1 = debug\n
                                          2 = verbosity\n
                                          3 = info\n
                                          4 = warn\n
                                          5 = error\n
         -log-file <server.log>       Sets the filename of the log\n
         -script-file <script.as>     Server script to execute\n
         -print-stats                 Prints stats to the console\n
         -version                     Prints the server version numbers\n
         -fg                          Starts the server in the foreground (background by default)\n
         -resource-dir <path>         Sets the path to the resource directory\n
         -auth-file <server.auth>             Path to file with authorization info\n
         -motd-file <server.motd>             Path to file with message of the day\n
         -rules-file <server.rules>           Path to file with rules for this server\n
         -blacklist-file <server.blacklist>   Path to file where bans are persisted\n
         -vehicle-limit {{0-...}}       Sets the maximum number of vehicles that a user is allowed to have\n
         -owner <name|organisation>   Sets the owner of this server (for the !owner command) (optional)\n
         -website <URL>               Sets the website of this server (for the !website command) (optional)\n
         -irc <URL>                   Sets the IRC url for this server (for the !irc command) (optional)\n
         -voip <URL>                  Sets the voip url for this server (for the !voip command) (optional)\n
         -help                        Show this list\n");
    }

    pub fn show_version(&self) {
        println!(
            "Rigs of Rods Server\n
             * using Protocol {RORNET_VERSION}\n"
        );
    }
}