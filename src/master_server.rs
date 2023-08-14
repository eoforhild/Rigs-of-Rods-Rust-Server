use std::collections::HashMap;
use futures::executor::block_on;
use reqwest::{Method, StatusCode, Response, Error};

use crate::config::{Config, CONF};
use crate::logger::{LogLevel, self};

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

    // Registers server on the server list
    pub fn register(&mut self) -> bool {
        let mut data = HashMap::new();
        let conf: &Config = unsafe { CONF.as_ref().unwrap() };

        let port = conf.get_listen_port().to_string();
        let max_clients = conf.get_max_clients().to_string();
        let password = (conf.is_public() as u32).to_string();

        data.insert("ip", conf.get_ip_addr());
        data.insert("port", &port);
        data.insert("name", conf.get_server_name());
        data.insert("terrain-name", conf.get_terrain_name());
        data.insert("max-clients", &max_clients);
        data.insert("version", "RoRnet_2.44");
        data.insert("use-password", &password);

        self.m_server_path = Some(format!("https://{}/server-list", conf.get_serverlist_path()));
        
        // Attempt to register onto the server list
        logger::log(LogLevel::Info, 
            &format!("Attempting to register on serverlist {}", self.m_server_path.as_ref().unwrap()));
        let response = match block_on(self.http_request(Method::POST, data)) {
            Ok(res) => res,
            Err(err) => {
                logger::log(LogLevel::Error, &err.to_string());
                return false
            },
        };
        let stat_code = response.status().as_u16();
        if stat_code != 200 {
            logger::log(LogLevel::Error, 
                &format!("Registration failed, server responded with code {}", stat_code));
            return false;
        }

        // if success
        // parse response json
        // if (todo!() /* fail condition */) {
        //     logger::log(LogLevel::Error,
        //         "registration failed, invalid server response (JSON parsing failed)");
        //     // log debug raw response
        //     return false;
        // }

        // set trust_level and token based on response from server
        true
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

    // change to return tuple (response, StatusCode)
    async fn http_request(
        &self, 
        method: Method, 
        payload: HashMap<&str, &str>,
    ) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let res: Response = client
            .request(method, self.m_server_path.as_ref().unwrap())
            .json(&payload)
            .send()
            .await?;

        Ok(res)
    }
}

pub async unsafe fn retrieve_public_ip() -> Result<(), Error> {
    let conf: &mut Config = CONF.as_mut().unwrap() ;
    let client = reqwest::Client::new();
    let url: String = format!("https://{}/get-public-ip", conf.get_serverlist_path());
    let ip = client.get(url).send()
        .await?
        .text()
        .await?;

    conf.set_ip_addr(&ip);
    Ok(())
}