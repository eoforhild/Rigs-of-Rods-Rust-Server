use serde::{Serialize, Deserialize};
use serde_repr::*;
use serde_big_array::BigArray;

use crate::config::{Config, CONF};

pub const RORNET_MAX_PEERS: u32 = 64;
pub const RORNET_MAX_MESSAGE_LENGTH: u32 = 8192;
pub const RORNET_LAN_BROADCAST_PORT: u32 = 13000;
pub const RORNET_MAX_USERNAME_LEN: u32 = 40;

pub const RORNET_VERSION: &str = "RoRnet_2.44";

/* Just RoRNet stuff all in one little module.
   Perhaps in the future move this into a messaging type of module
   just for the sole purpose of making and parsing packets. */

#[derive(Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub enum MessageType {
    Hello  = 1025,                // client sends its version as first message
    
    // Hello responses
    Full,                         // no more slots for us
    WrongPw,                      // server send that on wrong pw
    WrongVer,                     // wrong version
    Banned,                       // client not allowed to join
    Welcome,                      // we can proceed

    // Technical
    Version,                      // server responds with its version
    ServerSettings,               // server send client the terrain name: server_info_t
    UserInfo,                     // user data that is sent from the server to the clients
    MasterInfo,                   // master information response
    NetQuality,                   // network quality information
}

/* All struct representations for packets */

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub command: MessageType,
    pub source: i32,
    pub streamid: u32,
    pub size: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
    pub protocol_version: [u8; 20],
    #[serde(with = "BigArray")]
    pub terrain: [u8; 128],
    #[serde(with = "BigArray")]
    pub server_name: [u8; 128],                    
    pub has_password: u8,
    #[serde(with = "BigArray")]
    pub info: [u8; 4096],                             
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    uniqueid: u32,
    authstatus: i32,
    slotnum: i32,
    colournum: i32,

    #[serde(with = "BigArray")] username: [u8; RORNET_MAX_USERNAME_LEN as usize],
    #[serde(with = "BigArray")] usertoken: [u8; 40],
    #[serde(with = "BigArray")] serverpassword: [u8; 40],
                                language: [u8; 10],
                                clientname: [u8; 10],
                                clientversion: [u8; 25],
    #[serde(with = "BigArray")] clientguid: [u8; 40],
                                sessiontype: [u8; 10],
    #[serde(with = "BigArray")] sessionoptions: [u8; 128],
    

}

impl ServerInfo {
    pub fn build_packet() -> Vec<u8>{
        let conf: &Config = unsafe { CONF.as_ref().unwrap() };

        let mut protocol_version: [u8; 20] = [0; 20];
        let mut terrain: [u8; 128] = [0; 128];
        let mut server_name: [u8; 128] = [0; 128];
        let has_password: u8 = !(conf.get_public_pw().is_empty() as u8);
        let mut info: [u8; 4096] = [0; 4096];

        cpy_str(&mut protocol_version, RORNET_VERSION);
        cpy_str(&mut terrain, conf.get_terrain_name());
        cpy_str(&mut server_name, conf.get_server_name());
        // TODO! Get Motd
        
        bincode::serialize(&ServerInfo {
            protocol_version: protocol_version,
            terrain: terrain,
            server_name: server_name,
            has_password,
            info: info,
        }).unwrap()
    }
}

// Copies a string to a byte array
fn cpy_str(dst: &mut [u8], src: &str) {
    for (i, c) in src.bytes().enumerate() {
        if i >= dst.len() { break; }
        dst[i] = c;
    }
}