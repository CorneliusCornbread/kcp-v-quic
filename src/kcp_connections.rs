use std::{io::Write, net::{UdpSocket, SocketAddr}};
use kcp::Kcp;

pub const KCP_CONV: u32 = 12;

pub struct WriteableUdpSocket {
    pub raw_socket: UdpSocket
}

impl WriteableUdpSocket {
    pub fn new_connection(address: SocketAddr) -> anyhow::Result<WriteableUdpSocket> {
        let writeable = WriteableUdpSocket {
            raw_socket: UdpSocket::bind(address)?,
        };

        Ok(writeable)
    }
}

impl Write for WriteableUdpSocket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.raw_socket.send(buf);
        Ok(8)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}


pub struct KcpSocket {
    pub kcp_connection: Kcp<WriteableUdpSocket>
}

impl Default for KcpSocket {
    fn default() -> Self {
        Self { socket: Udp, kcp_connection: todo!()  }
    }
}

impl Write for KcpSocket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut output = self.data.lock().expect("Unable to get mutex, is somethign else using the mutex on this thread?");
        output.write(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut output = self.data.lock().expect("Unable to get mutex, is somethign else using the mutex on this thread?");
        output.flush()?;
        Ok(())
    }
}

pub struct KcpServer {
    pub kcp_connection: Kcp<KcpSocket>

}

impl KcpServer {
    pub fn open_rec_stream(&self) {
        
    }

    fn spawn_rec_thread(&self) {
        let mut buf = Vec::new();
        self.kcp_connection.recv(&mut buf);
    }
}

impl Default for KcpServer {
    fn default() -> Self {
        Self { kcp_connection: Kcp::new_stream(KCP_CONV, Default::default()) }
    }
}

pub struct KcpConnections {
    pub client: Kcp<KcpSocket>,
    pub server: KcpServer,

    pub client_send: Vec<u8>,
    pub server_send: Vec<u8>,
}

impl KcpConnections {
    pub fn start_client_server(&self) {
        
    }

    pub fn run(&self) -> Result<(), String> {
        todo!()
    }
}