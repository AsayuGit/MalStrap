extern crate tree_magic;

use std::env;
use std::path::Path;
use std::process::exit;
use std::fs;
use std::io;

fn print_usage() { 
    let args: Vec<String> = env::args().collect();
    let path: &Path = Path::new(args[0].as_str());
    let filename: &str = path.file_name().unwrap().to_str().unwrap(); 

    println!("Usage : {} <project> <sample>", filename);
}

fn create_files(project_name: &String, sample_name: &String) -> io::Result<()> {
    let sample_type: String = tree_magic::from_filepath(Path::new(sample_name));
    let sample_dir: String = project_name.to_owned() + "/" + sample_type.as_str();

    println!("Saving sample to {}", sample_dir);

    fs::create_dir_all(sample_dir)?;

    return Ok(()); 
}

fn create_notes(project_name: &String) {

}

fn create_scripts(project_name: &String) {

}

fn create_project_folder(project_name: &String, sample_name: &String) -> io::Result<()> {
    if Path::new(project_name).is_dir() {
        println!("Project folder {} already exists, override [y/N] ? ", project_name);

        let mut user_input: String = String::new();
        io::stdin().read_line(&mut user_input)?;

        match user_input.to_lowercase().trim() {
            "y" => {
                println!("Overrride !");
                fs::remove_dir_all(project_name)?;
            },
            _ => {
                println!("Aborting");
                return Ok(());
            },
        }
    }
    
    println!("Bootstraping project {} ...", project_name);

    fs::create_dir(project_name)?;
    create_files(project_name, sample_name)?;
    create_notes(project_name);
    create_scripts(project_name);

    return Ok(());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    println!("MalStrap : The malware analysis bootstraping tool");

    if args.len() < 3 {
        print_usage();
        exit(-1);
    }

    let _ = create_project_folder(&args[1], &args[2]);
}
