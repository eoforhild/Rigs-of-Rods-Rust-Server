use std::collections::HashMap;
use reqwest::{Method, StatusCode};

use crate::config::{Config, CONF};
use crate::logger::{LogLevel,  self};

struct Client {
    m_token: Option<String>,
    m_trust_level: i32,
    m_is_registered: bool,
    m_server_path: Option<String>
}

impl Client {
    pub fn new() -> Client {
        Client {
            m_token: None,
            m_trust_level: -1,
            m_is_registered: false,
            m_server_path: None,
        }
    }

    pub fn register(&self) -> bool {
        let mut data = HashMap::new();
        let conf: &Config = unsafe { CONF.as_ref().unwrap() };

        let port = conf.get_listen_port().to_string();
        let max_clients = conf.get_max_clients().to_string();
        data.insert("ip", conf.get_ip_addr());
        data.insert("port", &port);
        data.insert("name", conf.get_server_name());
        data.insert("terrain-name", conf.get_terrain_name());
        data.insert("max-clients", &max_clients);
        data.insert("version", "RoRnet_2.44");
        data.insert("use-password", &(conf.is_public() as u32).to_string());

        let m_server_path: String = String::from("/") + 
            conf.get_serverlist_path() + "/server-list";
        
        logger::log(LogLevel::Info, 
            &format!("Attempting to register on serverlist {}", m_server_path));
        
        
        true
    }

    pub fn unregister(&self) -> bool {
        todo!()
    }

    // Change user_list to utilize json
    pub fn send_heartbeat(&self, user_list: &str) -> bool {
        todo!()
    }

    pub fn is_registered(&self) -> bool { self.m_is_registered }

    pub fn get_trust_level(&self) -> i32 { self.m_trust_level }

    async fn http_request(
        &self, 
        method: Method, 
        payload: HashMap<&str, &str>,
    ) -> Result<StatusCode, Error>{
        let conf = unsafe { CONF.as_ref().unwrap() };
        let client = reqwest::Client::new();
        let res = client
            .request(method, conf.get_serverlist_host())
            .json(&payload)
            .send()
            .await?;

        res.status()
    }
}