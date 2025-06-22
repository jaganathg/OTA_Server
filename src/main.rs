mod checksum;
mod cli;
mod config;
mod handlers;
mod mdns;
mod metadata;
mod metadata_manager;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use config::ServerConfig;
use handlers::{health, kernels, version};
use mdns::MdnsServiceWrapper;
use metadata_manager::MetadataManager;
use tracing_subscriber::fmt::init;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<()> {
    init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Start { config } => {
            start_server(config).await?;
        }
        Commands::AddKernel {
            version,
            file,
            description,
            config,
        } => {
            add_kernel_command(config, version, file, description).await?;
        }
        Commands::List { config } => {
            list_kernels_command(config).await?;
        }
    }

    Ok(())
}

async fn start_server(config_path: String) -> Result<()> {
    let config = ServerConfig::load_from_file(&config_path)
        .await
        .unwrap_or_else(|_| {
            println!("Using default configuration");
            ServerConfig::default()
        });

    config.ensure_directories().await?;

    let addr = ([0, 0, 0, 0], config.server.port);

    let routes = health()
        .or(version(config.clone()))
        .or(kernels(config.clone()));

    println!(
        "OTA Server running on http://{}:{}",
        config.server.host, config.server.port
    );
    println!("Kernels directory: {}", config.paths.kernels_dir);
    println!("Metadata directory: {}", config.paths.metadata_dir);

    // Start mDNS service advertisement
    let mut mdns_service = MdnsServiceWrapper::new(config.server.port, &config.server.host)?;
    mdns_service.start().await?;

    println!("mDNS service started - advertising as _ota._tcp.local");

    warp::serve(routes).run(addr).await;

    Ok(())
}

async fn add_kernel_command(
    config_path: String,
    version: String,
    file: String,
    description: String,
) -> Result<()> {
    let config = ServerConfig::load_from_file(&config_path).await?;
    config.ensure_directories().await?;

    let manager = MetadataManager::new(config.paths.kernels_dir, config.paths.metadata_dir);

    manager
        .add_kernel(version.clone(), file, description)
        .await?;
    println!("Successfully added kernel version: {}", version);

    Ok(())
}

async fn list_kernels_command(config_path: String) -> Result<()> {
    let config = ServerConfig::load_from_file(&config_path).await?;

    let manager = MetadataManager::new(config.paths.kernels_dir, config.paths.metadata_dir);

    let history = manager.list_versions().await?;

    println!("Available kernel versions:");
    println!("Latest: {}", history.latest);
    println!();

    for kernel in &history.versions {
        println!("Version: {}", kernel.version);
        println!("  File: {}", kernel.kernel_file);
        println!("  Size: {} bytes", kernel.file_size);
        println!(
            "  Date: {}",
            kernel.release_date.format("%Y-%m-%d %H:%M:%S UTC")
        );
        println!("  Description: {}", kernel.description);
        println!();
    }

    Ok(())
}
