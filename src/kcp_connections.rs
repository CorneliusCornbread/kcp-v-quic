use tokio_kcp::KcpStream;

pub const KCP_CONV: u32 = 12;

pub struct KcpConnections {
    pub client: KcpStream,
    pub server: KcpStream,
}
