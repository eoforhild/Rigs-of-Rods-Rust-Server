use crate::Config;

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
        todo!()
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
}