use super::Config;

use crate::net::RORNET_VERSION;

impl Config {
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