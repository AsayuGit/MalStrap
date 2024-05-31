use std::collections::HashMap;

use crate::config::PluginConfig;
use crate::{Args, Commands};
use crate::project_manager::ProjectManager;
use crate::sample::Sample;

pub struct CLI {
    project: ProjectManager,
}

impl CLI {
    pub fn new(project: ProjectManager) -> Self {
        return Self {
            project,
        };
    }

    pub fn run(&mut self, args: Args) {
        match &args.commands {
            Some(Commands::Sample { name, show, tag, remove_tag }) => {
                // Add a new tag to a sample
                if let Some(tag) = tag {
                    if let Some(sample) = self.project.get_sample(name) {
                        sample.add_tag(&tag);
                        self.project.save();
                    } else {
                        panic!("Cannot add tag {} because sample {} doesn't exists", tag, name);
                    }
                }
                
                // Remove a tag from a sample
                if let Some(tag) = remove_tag {
                    if let Some(sample) = self.project.get_sample(name) {
                        sample.remove_tag(&tag);
                        self.project.save();
                    } else {
                        panic!("Cannot remove tag {} because sample {} doesn't exists", tag, name);
                    }
                }
                
                if *show {
                    if let Some(sample) = self.project.get_sample(name) {
                        println!("Summary of sample \"{}\" :\n{}", name, sample);
                    } else {
                        panic!("No such sample \"{}\"", name);
                    }
                }
            },
            Some(Commands::Project { list, add, del }) => {
                if let Some(sample_path) = add {
                    println!("Adding {} ...", sample_path);
        
                    self.project.add_sample(&sample_path);
                }
                if let Some(sample_name) = del {
                    println!("Deleting {} ...", sample_name);
        
                    self.project.del_sample(&sample_name);
                }
                if *list {
                    let sample_list: &HashMap<String, Sample> = self.project.list_samples();
                    if sample_list.is_empty() {
                        println!("There is no samples to list.");
                    } else {
                        println!("Listing samples !");
                        for (_, sample) in sample_list {
                            println!("{} [{}] -> {}", sample.name, sample.magic, sample.path);
                        }
                    }
                }
            },
            Some(Commands::Notes { generate }) => {
                if *generate {
                    self.project.compile_notes();
                }
            },
            Some(Commands::Config { show, vt_toogle }) => {
                let plugin_config: PluginConfig = self.project.get_plugin_config();

                if *vt_toogle {
                    self.project.vt_enable(!plugin_config.virus_total);
                    self.project.save();
                }

                if *show {
                    println!("VirusTotal plugin : {}", if plugin_config.virus_total { "Enabled" } else { "Dissabled" });
                }
            },
            _ => {},
        }
    }
}