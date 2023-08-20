use serde::{Serialize, Deserialize};
use serde_repr::*;

use crate::config::{Config, CONF};

pub static RORNET_MAX_PEERS: u32 = 64;
pub static RORNET_MAX_MESSAGE_LENGTH: u32 = 8192;
pub static RORNET_LAN_BROADCAST_PORT: u32 = 13000;
pub static RORNET_MAX_USERNAME_LEN: u32 = 40;

pub static RORNET_VERSION: &str = "RoRnet_2.44";

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
pub struct ServerInfo<'a> {
    protocol_version: &'a [u8],                 // Must be 20 bytes
    terrain: &'a [u8],                          // Must be 128 bytes
    server_name: &'a [u8],                      // Must be 128 bytes
    has_password: u8,
    info: &'a [u8],                             // Must be 4096 bytes
}

impl ServerInfo<'_> {
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
            protocol_version: &protocol_version,
            terrain: &terrain,
            server_name: &server_name,
            has_password,
            info: &info,
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