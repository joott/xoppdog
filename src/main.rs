use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};

mod commands;
use commands::{
    start_sit,
    shake_figure,
    fetch_figure,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start watching for file modifications and update tex files
    Sit(SitArgs),

    /// Modify a tracked xopp
    Fetch(FetchArgs),

    /// Create a tracked xopp
    Shake(ShakeArgs),
}

#[derive(Args)]
struct SitArgs {
    /// Figure directory
    root: PathBuf,
}

#[derive(Args)]
struct FetchArgs {
    /// Figure directory
    root: PathBuf,
}

#[derive(Args)]
struct ShakeArgs {
    /// Name of figure
    name: String,

    /// Figure directory
    root: PathBuf,
}

fn sit_command(args: SitArgs) {
    let root = args.root;
    start_sit(root);
}

fn fetch_command(args: FetchArgs) {
    let root = args.root;
    fetch_figure(root);
}

fn shake_command(args: ShakeArgs) {
    let name = args.name;
    let root = args.root;
    shake_figure(name, root);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sit(args) => sit_command(args),
        Commands::Shake(args) => shake_command(args),
        Commands::Fetch(args) => fetch_command(args),
    };
}
