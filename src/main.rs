extern crate tree_magic;

#[macro_use]
extern crate ini;

use clap::{Parser, Subcommand};

mod config;
mod project_manager;
use project_manager::ProjectManager;

mod cli;
use cli::CLI;

mod sample;

#[derive(Subcommand, Debug)]
enum Commands {
    /// Project init
    Init {
        /// Project path
        path: String,
    },

    /// Project actions
    Project {
        /// List all samples
        #[arg(short, long)]
        list: bool,

        /// Add a new sample to the project
        #[arg(short, long, value_name = "PATH")]
        add: Option<String>,
        
        /// Delete a sample from the project
        #[arg(short, long, value_name = "PATH")]
        del: Option<String>,
    },

    /// Sample actions
    Sample {
        /// Sample name
        name: String,

        /// List essential info about a sample
        #[arg(short, long)]
        show: bool,

        /// Add a custom tag to a sample
        #[arg(short, long, value_name = "TAG")]
        tag: Option<String>,

        /// Remove a tag from a sample
        #[arg(short, long, value_name = "TAG")]
        remove_tag: Option<String>,
    },

    /// Configure workspace settings
    Config {
        /// Show the current config
        #[arg(short, long)]
        show: bool,

        /// Enable/Disable the VirusTotal plugin
        #[arg(short, long)]
        vt_toogle: bool,
    },
}

/// MalStrap : The malware analysis bootstraping tool
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    commands: Option<Commands>,
}

fn main() {
    let args: Args = Args::parse();

    let project: Option<ProjectManager> = match &args.commands {
        Some(Commands::Init { path }) => match ProjectManager::new(path) {
            Ok(p) => Some(p),
            Err(_) => {
                println!("Failed to init project !");
                None
            }
        },
        _ => match ProjectManager::open(".") {
            Ok(p) => Some(p),
            Err(_) => {
                println!("Current directory is not a MalStrap project directory.");
                None
            }
        }
    };

    if let Some(project) = project {
        let mut cli: CLI = CLI::new(project);
        cli.run(args);
    }
}
