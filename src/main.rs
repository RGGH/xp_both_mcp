use anyhow::Result;
use clap::{Parser, ValueEnum};
use common::counter::Counter;
use rmcp::ServiceExt;
use rmcp::transport::sse_server::SseServer;
use rmcp::transport::stdio;
use std::net::SocketAddr;
use tracing::{debug, error, info};
use tracing_subscriber::{self};
mod common;

/// RMCP server with support for both stdio and SSE transport
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Transport method to use
    #[arg(short, long, value_enum, default_value_t = TransportType::Sse)]
    transport: TransportType,

    /// Bind address for SSE server (only used with sse transport)
    #[arg(short, long, default_value = "127.0.0.1:8000")]
    bind_address: String,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[derive(Debug, Clone, ValueEnum)]
enum TransportType {
    /// Use standard input/output for transport
    Stdio,
    /// Use Server-Sent Events over HTTP for transport
    Sse,
}

/// Usage:
/// - For SSE (default): cargo run
/// - For SSE with custom address: cargo run -- -b 0.0.0.0:9000
/// - For stdio: cargo run -- --transport stdio
/// - Set log level: cargo run -- --log-level debug
#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize tracing subscriber with simple format
    let level = match args.log_level.as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    tracing_subscriber::fmt().with_max_level(level).init();

    info!("Starting RMCP server");
    debug!(transport = ?args.transport, bind_address = %args.bind_address, "Parsed command line arguments");

    match args.transport {
        TransportType::Stdio => {
            info!("Using stdio transport");

            // Create and serve the counter over stdio
            debug!("Initializing Counter service with stdio transport");
            let service = Counter::new()
                .serve(stdio())
                .await
                .inspect_err(|e| error!("Failed to serve Counter over stdio: {:?}", e))?;

            info!("Service initialized, waiting for completion");
            service.waiting().await?;
            info!("Service completed");
        }
        TransportType::Sse => {
            info!("Using SSE transport (default)");

            // Parse bind address
            debug!("Parsing bind address: {}", args.bind_address);
            let addr: SocketAddr = match args.bind_address.parse() {
                Ok(addr) => addr,
                Err(e) => {
                    error!("Failed to parse bind address: {}", e);
                    return Err(e.into());
                }
            };

            // Create and serve the counter over SSE
            info!("Starting SSE server on {}", addr);
            let ct = match SseServer::serve(addr).await {
                Ok(server) => {
                    debug!("SSE server started successfully");
                    server.with_service(Counter::new)
                }
                Err(e) => {
                    error!("Failed to start SSE server: {:?}", e);
                    return Err(e.into());
                }
            };

            // Wait for Ctrl+C signal
            info!("Server running, press Ctrl+C to stop");
            tokio::signal::ctrl_c().await?;
            info!("Shutting down SSE server");
            ct.cancel();
            info!("Server shutdown complete");
        }
    }

    info!("RMCP server exiting");
    Ok(())
}
