use futures::executor::block_on;
use reqwest::{Method, Response, Error};
use serde_json::{Value, json};

use crate::config::{Config, CONF};
use crate::logger::{LogLevel, self};

pub struct Client {
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

    /// Registers server on the server list
    pub fn register(&mut self) -> bool {
        let conf: &Config = unsafe { CONF.as_ref().unwrap() };

        let port: String = conf.get_listen_port().to_string();
        let max_clients: String = conf.get_max_clients().to_string();
        let password: String = (conf.is_public() as u32).to_string();
        let data: Value = json!({
            "ip": conf.get_ip_addr(),
            "port": &port,
            "name": conf.get_server_name(),
            "terrain-name": conf.get_terrain_name(),
            "max-clients": &max_clients,
            "version": "RoRnet_2.44",
            "use-password": &password
        });

        self.m_server_path = Some(format!("{}/server-list", conf.get_serverlist_path()));
        
        // Attempt to register onto the server list
        logger::log(LogLevel::Info, 
            &format!("Attempting to register on serverlist {}", self.m_server_path.as_ref().unwrap()));
        let response: Response = match block_on(self.http_request(Method::POST, data)) {
            Ok(res) => res,
            Err(err) => {
                logger::log(LogLevel::Error, &err.to_string());
                return false
            },
        };

        // Try to parse a registration error
        let stat_code: u16 = response.status().as_u16();
        if stat_code != 200 {
            let err_json: Value  = match block_on(response.json()) {
                Ok(res) => res,
                Err(err) => {
                    logger::log(LogLevel::Error, "Could not properly parse server response. Exiting...");
                    logger::log(LogLevel::Debug, &err.to_string());
                    return false;
                }
            };
            logger::log(LogLevel::Error, 
                &format!("HTTP {}: {}", stat_code, &err_json["message"]));
            return false;
        }

        // Try to parse a successful registration into a serde_json::Value
        match block_on(response.json::<Value>()) {
            Ok(res) => {
                let trust_level = &res["verified-level"].as_i64();
                let challenge = &res["challenge"].as_str();
                if trust_level.is_none() || challenge.is_none() {
                    logger::log(LogLevel::Error, "Registration failed. Server responded incorrectly.");
                    logger::log(LogLevel::Debug, 
                        &format!("Raw response: {}", &res.to_string()));
                    return false;
                }

                self.m_trust_level = trust_level.unwrap() as i32;
                self.m_token = Some(challenge.unwrap().to_string());
                self.m_is_registered = true;
                true
            },
            Err(err) => {
                logger::log(LogLevel::Error, "Could not properly parse server response. Exiting...");
                logger::log(LogLevel::Debug, &err.to_string());
                false
            },
        }
    }

    pub fn unregister(&self) -> bool {
        todo!()
    }

    // Change user_list to utilize json
    pub fn send_heartbeat(&self, user_list: &str) -> bool {
        // heartbeat consists of putting a json to the server with
        // challenge = self.m_token and users = user_list
        todo!()
    }

    pub fn is_registered(&self) -> bool { self.m_is_registered }

    pub fn get_trust_level(&self) -> i32 { self.m_trust_level }

    async fn http_request(
        &self, 
        method: Method, 
        payload: Value,
    ) -> Result<Response, Error> {
        let client: reqwest::Client = reqwest::Client::new();
        let res: Response = client
            .request(method, self.m_server_path.as_ref().unwrap())
            .json(&payload)
            .send().await?;

        Ok(res)
    }
}

/// Retrieves public ip of the computer hosting this server
/// and mutates the global config accordingly to this new ip.
pub async unsafe fn retrieve_public_ip() -> Result<(), Error> {
    let conf: &mut Config = CONF.as_mut().unwrap() ;
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = format!("{}/get-public-ip", conf.get_serverlist_path());
    let ip: String = client
        .get(url)
        .send().await?
        .text().await?;

    conf.set_ip_addr(&ip);
    Ok(())
}