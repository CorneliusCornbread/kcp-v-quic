use std::net::SocketAddr;

use benchmark_rs::benchmarks::Benchmarks;
use benchmark_rs::stopwatch::StopWatch;
use kcp_connections::KcpConnections;
use tokio::runtime::Runtime;
use tokio_kcp::{KcpConfig, KcpStream};
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

    benchmarks.run()?;

    let csv = benchmarks.summary_as_csv(true, false);

    Ok(())
}

fn run_kcp_benchmarks(
    stop_watch: &mut StopWatch,
    config: &str,
    work: u64,
) -> Result<(), anyhow::Error> {
    let runtime = Runtime::new().unwrap();
    let handle = runtime.handle();
    handle.block_on(async_run(stop_watch, config, work))?;
    Ok(())
}

async fn async_run(
    stop_watch: &mut StopWatch,
    config: &str,
    work: u64,
) -> Result<(), anyhow::Error> {
    let client_message = "Haha yes, I am a client. asldkvjalkjvlkasdjflksamvaksmv".as_bytes();
    let server_message = "Hmm ahh, hmm yes I am a server. asjlkdjvlkajdflkajsfdkljaslkd".as_bytes();

    let ip = "127.0.0.1:35600";

    let config = KcpConfig::default();

    let addr = ip.parse::<SocketAddr>().unwrap();

    let mut kcp_connections = KcpConnections {
        client: KcpStream::connect(&config, addr).await.unwrap(),
        server: KcpStream::connect(&config, addr).await.unwrap(),
    };

    Ok(())
}
