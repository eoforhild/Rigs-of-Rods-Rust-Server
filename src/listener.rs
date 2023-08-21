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
    ServerInfo
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
    clients: Arc<TokioMutex<Vec<Client>>>,
}

impl Listener {
    pub fn new(config: &Config) -> Listener {
        Listener {
            ip: "0.0.0.0".to_string(),
            port: "12456".to_string(),
            tick: 64,
            clients: Arc::new(TokioMutex::new(Vec::new()))
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
                        self.process_client_data(&sock, &src_addr, &data).await?;
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

    pub async fn process_client_data(&self, socket: &Arc<TokioMutex<UdpSocket>>, src_addr: &std::net::SocketAddr, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Handle client data here
        // For example, update client state, send responses, etc.
        // Make sure client sent a packet at least the size of header
        if data.len() < 16 {
            return Err("Packet smaller than header size".into());
        }
        let (head_raw, payload_raw) = (&data[..16], &data[16..]);
        let head = bincode::deserialize::<Header>(head_raw).unwrap();

        // Add or update client in the clients list
        /* TODO! Maybe use a hashmap instead for quicker access? Client list isn't going to be
            that large so memory shouldn't be an issue */
        let mut clients = self.clients.lock().await;
        if let Some(client) = clients.iter_mut().find(|c| c.ipaddr == *src_addr) {
            // Update existing client
            match client.state {
                ClientState::Pending => {
                    if head.command == MessageType::UserInfo {
                        client.state = ClientState::Connected;
                        logger::log(LogLevel::Debug, 
                            &format!("Client {} moved from pending to connected", &client.ipaddr.to_string()));
                        // Update the client struct with stuff idk
                    }
                },
                ClientState::Connected => {
                    // Actually do the client stuff in game here
                },
            };
        } else {
            // Add new client, make sure the client sends HELLO as the first packet
            if head.command == MessageType::Hello {
                // Create a new client in the Pending state
                clients.push(Client { state: ClientState::Pending, ipaddr: *src_addr });
                logger::log(LogLevel::Debug, &format!("New client in pending: {}", src_addr));
                // Sends a ServerInfo packet back to the client
                let s_info: Vec<u8> = ServerInfo::build_packet();
                self.send(socket, s_info, *src_addr).await?;
            } else {
                logger::log(LogLevel::Warn, 
                    &format!("Client with IP {} did not send a HELLO packet as its first packet", src_addr));
            }
        }
        Ok(())
    }

    pub async fn send(&self, socket: &Arc<TokioMutex<UdpSocket>>, data: Vec<u8>, dest: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        socket.lock().await.send_to(&data, dest).await?;
        Ok(())
    }

    fn buf_to_str(buf: &[u8]) -> &str {
        str::from_utf8(buf).unwrap()
    }
}
