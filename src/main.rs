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
    }
}

/// MalStrap : The malware analysis bootstraping tool
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// List all samples
    #[arg(short, long)]
    list: bool,

    /// Add a new sample to the project
    #[arg(short, long, value_name = "PATH")]
    add: Option<String>,
    
    /// Delete a sample from the project
    #[arg(short, long, value_name = "PATH")]
    del: Option<String>,

    #[command(subcommand)]
    commands: Option<Commands>,
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
