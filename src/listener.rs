use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::str;
use tokio::signal;
use tokio::sync::{broadcast, Mutex as TokioMutex};
use tokio::net::UdpSocket;
use tokio::time::{self, Interval, interval};
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt;

use crate::net::{
    self,
    MessageType,
    Header,
    ServerInfo,
    RORNET_VERSION
};

use crate::config::Config;
use crate::logger::{
    self,
    LogLevel
};

enum ClientState {
    Pending,
    Connected
}

pub struct Client {
    state: ClientState,
    ipaddr: std::net::SocketAddr,
}

pub struct Listener {
    ip: String,
    port: String,
    tick: u64,
    clients: Arc<TokioMutex<HashMap<std::net::SocketAddr , Client>>>,
}

impl Listener {
    pub fn new() -> Listener {
        Listener {
            ip: "0.0.0.0".to_string(),
            port: "12456".to_string(),
            tick: 64,
            clients: Arc::new(TokioMutex::new(HashMap::new()))
        }
    }
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.ip, self.port);

        let sock = UdpSocket::bind(&addr).await?;
        logger::log(LogLevel::Info, &format!("Server listening on {}", addr));

        let sock: Arc<TokioMutex<UdpSocket>> = Arc::new(TokioMutex::new(sock));
        let signal_sock: Arc<TokioMutex<UdpSocket>> = sock.clone();

        let sigint = signal::ctrl_c();
        tokio::pin!(sigint);

        // setup tick rate - todo not finished
        let tick_interval = Duration::from_millis(100);
        let interval = interval(tick_interval);

        let tick_stream = IntervalStream::new(interval);
        let mut tick_stream = tick_stream.fuse();
        loop {
            tokio::select! {
                _ = &mut sigint => {
                    println!("Received Ctrl+C, shutting down...");
                    break;
                }
                // _ = tick_stream.next() => {
                //    self.process_tick(&sock, &sender).await?;
                // }
                client_data = self.receive_client_data(&sock) => {
                    if let Ok((data, src_addr)) = client_data {
                        self.process_client_data(&sock, src_addr, &data).await?;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn process_tick(&self, socket: &Arc<TokioMutex<UdpSocket>>, sender: &broadcast::Sender<()>) -> Result<(), Box<dyn std::error::Error>> {
        sender.send(())?;

        // Perform tick-based processing here

        Ok(())
    }

    pub async fn process_message(&self, socket: &Arc<TokioMutex<UdpSocket>>) -> Result<(), Box<dyn std::error::Error>> {
        // Perform message processing here

        Ok(())
    }

    pub async fn receive_client_data(&self, socket: &Arc<TokioMutex<UdpSocket>>) -> Result<(Vec<u8>, std::net::SocketAddr), Box<dyn std::error::Error>> {
        // buf is the size of Header + max size of message
        let mut buf = [0u8; (16 + net::RORNET_MAX_MESSAGE_LENGTH as usize)];
        let (size, src_addr) = socket.lock().await.recv_from(&mut buf).await?;
        let data = buf[..size].to_vec();
        Ok((data, src_addr))
    }

    pub async fn process_client_data(&self, socket: &Arc<TokioMutex<UdpSocket>>, src_addr: std::net::SocketAddr, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Handle client data here
        // For example, update client state, send responses, etc.
        // Make sure client sent a packet at least the size of header
        if data.len() < 16 {
            logger::log(LogLevel::Debug,
                &format!("Client {} sent a non-RoRnet packet. Ignoring...", &src_addr));
            return Ok(());
        }
        let (head_raw, payload_raw) = (&data[..16], &data[16..]);
        let head = match bincode::deserialize::<Header>(head_raw) {
            Ok(res) => res,
            Err(_) => {
                logger::log(LogLevel::Debug,
                &format!("Client {} sent a non-RoRnet packet. Ignoring...", &src_addr));
                return Ok(());
            },
        };

        // Add or update client in the clients list
        let mut clients = self.clients.lock().await;
        if let Some(client) = clients.get_mut(&src_addr) {
            // Update existing client
            match client.state {
                ClientState::Pending => {
                    if head.command == MessageType::UserInfo {
                        client.state = ClientState::Connected;
                        logger::log(LogLevel::Debug, 
                            &format!("Client {} moved from pending to connected", src_addr));
                        // Update the client struct with stuff idk
                    } else {
                        logger::log(LogLevel::Debug,
                            &format!("Client {} did not respond with a UserInfo packet, dropping connection...", src_addr));
                        clients.remove(&src_addr);
                    }
                },
                ClientState::Connected => {
                    // Actually do the client stuff in game here
                },
            }
        } else {
            // Make sure the client sends HELLO as the first packet
            if head.command != MessageType::Hello {
                logger::log(LogLevel::Warn, 
                    &format!("Client {} did not send a HELLO packet as its first packet", src_addr));
                self.send(socket, MessageType::WrongVer, 0, 0, vec![], src_addr).await?;
                return Ok(());
            }
            if buf_to_str(&payload_raw) != RORNET_VERSION {
                logger::log(LogLevel::Warn, 
                    &format!("Client {} had wrong protocol version", src_addr));
                self.send(socket, MessageType::WrongVer, 0, 0, vec![], src_addr).await?;
                return Ok(());
            }
            // Creates a new client in the Pending state
            clients.insert(src_addr, Client { state: ClientState::Pending, ipaddr: src_addr });
            logger::log(LogLevel::Debug, &format!("New client in pending: {}", src_addr));
            // Sends a ServerInfo packet back to the client
            let s_info: Vec<u8> = ServerInfo::build_packet();
            self.send(socket, MessageType::Hello, 0, 0, s_info, src_addr).await?;
        }
        Ok(())
    }

    // Sends a payload with specified command. Make payload using build_packet
    // functions in net.rs
    pub async fn send(
        &self,
        socket: &Arc<TokioMutex<UdpSocket>>,
        command: MessageType,
        source: i32,
        streamid: u32,
        mut payload: Vec<u8>,
        dest: std::net::SocketAddr
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut msg: Vec<u8> = Header::build_packet(command, source, streamid, payload.len() as u32);
        msg.append(&mut payload);
        socket.lock().await.send_to(&msg, dest).await?;
        Ok(())
    }
}

fn buf_to_str(buf: &[u8]) -> &str {
    str::from_utf8(buf).unwrap()
}