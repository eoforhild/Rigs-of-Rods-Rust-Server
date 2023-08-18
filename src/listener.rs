use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::signal;
use tokio::sync::{broadcast, Mutex as TokioMutex};
use tokio::net::UdpSocket;
use tokio::time::{self, Interval, interval};
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt;

use crate::config::Config;

pub struct Client {
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
            port: config.get_listen_port().to_string(),
            tick: 64,
            clients: Arc::new(TokioMutex::new(Vec::new()))
        }
    }
    pub async fn run(&self, sender: broadcast::Sender<()>) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.ip, self.port);

        let sock = UdpSocket::bind(&addr).await?;
        println!("Server listening on {}", addr);

        let sock = Arc::new(Mutex::new(sock));
        let signal_sock = sock.clone();

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
                _ = tick_stream.next() => {
                    self.process_tick(&sock, &sender).await?;
                }
                message = sender.recv() => {
                    if let Ok(_) = message {
                        self.process_message(&sock).await?;
                    }
                }
                client_data = self.receive_client_data(&sock) => {
                    if let Ok((data, src_addr)) = client_data {
                        self.process_client_data(&sock, &src_addr, &data).await?;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn process_tick(&self, socket: &Arc<Mutex<UdpSocket>>, sender: &broadcast::Sender<()>) -> Result<(), Box<dyn std::error::Error>> {
        sender.send(())?;

        // Perform tick-based processing here

        Ok(())
    }

    pub async fn process_message(&self, socket: &Arc<Mutex<UdpSocket>>) -> Result<(), Box<dyn std::error::Error>> {
        // Perform message processing here

        Ok(())
    }

    pub async fn receive_client_data(&self, socket: &Arc<Mutex<UdpSocket>>) -> Result<(Vec<u8>, std::net::SocketAddr), Box<dyn std::error::Error>> {
        let mut buf = [0u8; 1024];
        let (size, src_addr) = socket.lock().await.recv_from(&mut buf).await?;
        let data = buf[..size].to_vec();
        Ok((data, src_addr))
    }

    pub async fn process_client_data(&self, socket: &Arc<Mutex<UdpSocket>>, src_addr: &std::net::SocketAddr, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Handle client data here
        // For example, update client state, send responses, etc.

        // Add or update client in the clients list
        let mut clients = self.clients.lock().await;
        if let Some(client) = clients.iter_mut().find(|c| c.ipaddr == *src_addr) {
            // Update existing client
        } else {
            // Add new client
            clients.push(Client { ipaddr: *src_addr });
            println!("New client connected: {}", src_addr);
        }

        // Send response
        let response = format!("Hello from server!");
        self.send(socket, response.as_bytes(), *src_addr).await?;

        Ok(())
    }

    pub async fn send(&self, socket: &Arc<Mutex<UdpSocket>>, data: Vec<u8>, dest: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        socket.lock().await.send_to(&data, dest).await?;
        Ok(())
    }
}
