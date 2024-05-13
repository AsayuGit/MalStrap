extern crate tree_magic;

#[macro_use]
extern crate ini;

use clap::Parser;

mod config;
mod project_manager;
use project_manager::ProjectManager;

mod cli;
use cli::CLI;

mod sample;

/// MalStrap : The malware analysis bootstraping tool
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to the project directory to init
    #[arg(short, long)]
    init: Option<String>,

    /// List all samples
    #[arg(short, long)]
    list: bool,

    /// Add a new sample to the project
    #[arg(short, long)]
    add: Option<String>,

    /// Delete a sample from the project
    #[arg(short, long)]
    del: Option<String>,

    /// List essential info about a sample
    #[arg(short, long)]
    summarize: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    let project_path: String = ".malstrap".to_string();
    let project: ProjectManager = match ProjectManager::open(&project_path) {
        Ok(p) => p,
        Err(_) => match ProjectManager::new(&project_path) {
            Ok(p) => p,
            Err(_) => {
                return;
            }
        }
    };

    println!("Project {} loaded successfuly !", project.get_name());
    let mut cli: CLI = CLI::new(project);
    cli.run(args);
}
