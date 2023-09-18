use benchmark_rs::benchmarks::Benchmarks;
use benchmark_rs::stopwatch::StopWatch;
use kcp::Kcp;
use kcp_connections::{KcpConnections, KcpSession};
mod kcp_connections;
mod quic_connections;

fn main() -> Result<(), anyhow::Error> {
    let packet_sizes = Vec::<u64>::new();
    let repeat = 10;

    let mut benchmarks = Benchmarks::new("KCP Performance Tests");
    benchmarks.add(
        "A Simple Benchmark",
        run_kcp_benchmarks,
        "",
        packet_sizes,
        repeat,
        1,
    )?;
    let csv = benchmarks.summary_as_csv(true, false);

    Ok(())
}

fn run_kcp_benchmarks(
    stop_watch: &mut StopWatch,
    config: &str,
    work: u64,
) -> Result<(), anyhow::Error> {
    let client_message = "Haha yes, I am a client".as_bytes();
    let server_message = "Hmm ahh, hmm yes I am a server".as_bytes();

    let mut kcp_connections = KcpConnections {
        client: KcpSession::new_bind("127.0.0.1:35601")
            .expect("Server unable to bind to local host on port 35601, is this port taken?"),
        server: KcpSession::new_bind("127.0.0.1:35600")
            .expect("Server unable to bind to local host on port 35600, is this port taken?"),
        client_send: Vec::from(client_message),
        server_send: Vec::from(server_message),
    };

    kcp_connections.start_client_server();

    Ok(())
}
