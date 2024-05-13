use std::collections::HashMap;

use crate::Args;
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
        if let Some(sample_path) = args.add {
            println!("Adding {} ...", sample_path);

            self.project.add_sample(&sample_path);
        }
        if let Some(sample_name) = args.del {
            println!("Deleting {} ...", sample_name);

            self.project.del_sample(&sample_name);
        }

        if args.list {
            println!("Listing samples !");

            let sample_list: &HashMap<String, Sample> = self.project.list_samples();
            for (_, sample) in sample_list {
                println!("{} [{}] -> {}", sample.name, sample.magic, sample.path);
            }
        }

        if let Some(sample_name) = args.summarize {
            println!("Summary of sample {} :", sample_name);

            self.project.sample_info(sample_name.as_str());
            /*
            let sample_info: HashMap<String, Box<dyn Debug>> = self.project.sample_info(sample_name.as_str());
            for (info, data) in sample_info {
                println!("{} : {:#?}", info, data);
            }*/
        }
    }
}