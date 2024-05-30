extern crate dirs;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::config::Config;
use crate::sample::Sample;

pub struct ProjectManager {
    path: String,
    config: Config,
    config_path: String,
    _global_config: HashMap<String, HashMap<String, Option<String>>>,
}

impl ProjectManager {
    fn load_global_config() -> HashMap<String, HashMap<String, Option<String>>> {
        let global_config_path: PathBuf =  dirs::home_dir().expect("Couldn't fetch home directory.").join(".config/malstrap");
        let global_config_path_str: &str = global_config_path.to_str().unwrap();
        let global_config: HashMap<String, HashMap<String, Option<String>>> = match ini!(safe global_config_path_str) {
            Ok(config) => config,
            Err(_) => {
                if let Ok(default_config_file) = File::create(global_config_path_str) {
                    write!(&default_config_file, "[malstrap]\nvt_key=YOUR_KEY_HERE\n").expect("msg");
                }

                ini!(global_config_path_str)
            }
        };

        return global_config;
    }

    pub fn new(path: &str) -> Result<Self, String> {
        /*
        Initialize a new project.

        This will panic if the project's folder cannot be created.
        */

        let project_path: &Path = Path::new(path);
        let config_path: PathBuf = project_path.join("config.json");
        let global_config: HashMap<String, HashMap<String, Option<String>>> = Self::load_global_config();

        return match Self::create_project_folder(path) {
            Ok(()) => return Ok(Self {
                path: String::from(project_path.to_str().unwrap()),
                config: Config::new(config_path.to_str().unwrap(), project_path.file_name().unwrap().to_str().unwrap()),
                config_path: String::from(config_path.to_str().unwrap()),
                _global_config: global_config,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn open(path: &str) -> Result<Self, String> {
        /*
        Load a previously created project form disk.
        */
        let project_path: &Path = Path::new(path);
        let config_path: PathBuf = project_path.join("config.json");
        let global_config: HashMap<String, HashMap<String, Option<String>>> = Self::load_global_config();

        return match Config::load(config_path.to_str().unwrap()) {
            Ok(project_config) => Ok(Self {
                path: String::from(project_path.to_str().unwrap()),
                config: project_config,
                config_path: String::from(config_path.to_str().unwrap()),
                _global_config: global_config,
            }),
            Err(e) => Err(format!("Couldn't open project {} {}", path, e.to_string())),
        }
    }

    pub fn get_name(&self) -> &String {
        /*
        Return the name of the current project.
        */

        return &self.config.name;
    }

    fn create_project_folder(path: &str) -> Result<(), String> {
        /*
        Create the project folder.

        If attempting to overwrite an existing project, the user is prompt for confirmation.
        */

        let project_dir: &Path = Path::new(path);
        
        if project_dir.is_dir() {
            println!("Directory {} already exists, override [y/N] ? ", project_dir.to_str().unwrap());
    
            let mut user_input: String = String::new();
            if let Err(e) = io::stdin().read_line(&mut user_input) {
                return Err(format!("Couldn't read user inputs : {}", e.to_string()));
            }
    
            match user_input.to_lowercase().trim() {
                "y" => {
                    println!("Overrride ! Erasing {}", project_dir.to_str().unwrap());
                    if let Err(e) = fs::remove_dir_all(&project_dir) {
                        return Err(format!("Couldn't remove previous project dir : {}", e.to_string()));
                    }
                },
                _ => {
                    println!("Aborting");
                    return Err(String::from("Project dir already exists."));
                },
            }
        }
        
        if let Err(e) = fs::create_dir_all(&project_dir) {
            return Err(format!("Couldn't create project directory {} : {}", project_dir.to_str().unwrap(), e.to_string()));
        }
    
        return Ok(());
    }    

    pub fn _create_files(&self, project_name: &str, sample_name: &str) -> io::Result<()> {
        let sample_type: String = tree_magic::from_filepath(Path::new(sample_name));
        let sample_dir: String = project_name.to_owned() + "/" + sample_type.as_str();
    
        println!("Saving sample to {}", sample_dir);
    
        fs::create_dir_all(sample_dir)?;
    
        return Ok(()); 
    }

    pub fn list_samples(&self) -> &HashMap<String, Sample> {
        return &self.config.samples;
    }

    pub fn save(&self) {
        let _ = self.config.save(&self.config_path);
    }

    // TODO: use Path manipulation to be cross platform
    pub fn add_sample(&mut self, path: &str) {
        if let Ok(mut sample) = Sample::new(path, &self._global_config["malstrap"]["vt_key"]) {
            let sample_src_path: &Path = Path::new(path);

            // Create the sample directory.
            let sample_dst_path: String = self.path.to_owned() + "/" + sample.magic.as_str();
            fs::create_dir_all(&sample_dst_path).expect("Unable to create sample directory.");

            // Copy the sample to its destination.
            fs::copy(sample_src_path, sample_dst_path.to_owned() + "/" + sample.name.as_str()).expect("Cannot copy sample to destination.");
            sample.path = sample_dst_path;

            // Then store the path to the sample relative to the project directory.
            //let _sample_dir: String = sample.magic.to_owned() + "/" + sample.name.as_str();

            self.config.samples.insert(sample.name.to_owned(), sample);

            // Finally save the new config state.
            self.save();
        }
    }

    pub fn del_sample(&mut self, sample_name: &str) {
        if let Some(sample) = self.config.samples.get(sample_name) {
            // Delete the sample file
            fs::remove_file(self.path.to_owned() + "/" + sample.path.as_str()).expect("Couldn't delete sample.");

            // Remove the sample from the config
            self.config.samples.remove(sample_name);

            // Finally save the new config state.
            self.save();
        }
    }

    // Return a sample by name if available
    pub fn get_sample(&mut self, sample_name: &str) -> Option<&mut Sample> {
        return self.config.samples.get_mut(sample_name);
    }
}