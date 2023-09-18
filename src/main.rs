use std::sync::{Arc, Mutex};
use std::{thread, any};
use std::time::Duration;
use benchmark_rs::benchmarks::Benchmarks;
use benchmark_rs::stopwatch::StopWatch;
use kcp::Kcp;
use kcp_connections::{KcpConnections, KcpSocket};
mod kcp_connections;
mod quic_connections;

fn main() -> Result<(), anyhow::Error> {
    let packet_sizes = Vec::<u64>::new();
    let repeat = 10;

    let mut benchmarks = Benchmarks::new("KCP Performance Tests");
    benchmarks.add("A Simple Benchmark", run_kcp_benchmarks, "", packet_sizes, repeat, 1)?;
    let csv = benchmarks.summary_as_csv(true, false);

    Ok(())
}

fn run_kcp_benchmarks(stop_watch: &mut StopWatch, config: &str, work: u64) -> Result<(), anyhow::Error> {
    let client_output = KcpSocket::default();
    let server_output = KcpSocket::default();

    let client_message = "Haha yes, I am a client".as_bytes();
    let server_message = "Hmm ahh, hmm yes I am a server".as_bytes();

    let kcp_connections = KcpConnections{
        client: Kcp::new(12, client_output),
        server: todo!(),
        client_send: Vec::from(client_message),
        server_send: Vec::from(server_message),
    };

    

    Ok(())
}
