use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ota-server")]
#[command(about = "OTA Server for kernel updates")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the OTA server
    Start {
        /// Configuration file path
        #[arg(short, long, default_value = "config/server.toml")]
        config: String,
    },
    /// Add a new kernel version
    AddKernel {
        /// Kernel version (e.g., 1.0.0)
        #[arg(short, long)]
        version: String,
        /// Kernel file name
        #[arg(short, long)]
        file: String,
        /// Description of this version
        #[arg(short, long)]
        description: String,
        /// Configuration file path
        #[arg(short, long, default_value = "config/server.toml")]
        config: String,
    },
    /// List all kernel versions
    List {
        /// Configuration file path
        #[arg(short, long, default_value = "config/server.toml")]
        config: String,
    },
}