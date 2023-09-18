use anyhow::Result;
use kcp::Kcp;
use std::{
    io::Write,
    net::{ToSocketAddrs, UdpSocket},
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
    time::Duration,
};

pub const KCP_CONV: u32 = 12;

pub struct KcpSession {
    pub kcp: Arc<RwLock<Kcp<Vec<u8>>>>,
    pub rec_thread: Option<JoinHandle<()>>,
}

impl KcpSession {
    pub fn spawn_rec_thread(&mut self) {
        let socket_ref = self.kcp.clone();

        let thread_handle = thread::spawn(move || {
            loop {
                let mut buf = Vec::new();
                let mut socket = socket_ref.write().unwrap();
                let _ = socket.recv(&mut buf); //TODO: log errors
                thread::sleep(Duration::from_millis(1));
            }
        });

        self.rec_thread = Some(thread_handle);
    }

    pub fn new_bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        Ok(Self {
            kcp: Arc::new(RwLock::new(Kcp::<Vec<u8>>::new_stream(
                KCP_CONV,
                Vec::new(),
            ))),
            rec_thread: None,
        })
    }
}

pub struct KcpConnections {
    pub client: KcpSession,
    pub server: KcpSession,

    pub client_send: Vec<u8>,
    pub server_send: Vec<u8>,
}

impl KcpConnections {
    pub fn start_client_server(&mut self) {
        let client = &mut self.client;
        let server = &mut self.server;

        server.spawn_rec_thread();
        client.spawn_rec_thread();
    }
}
