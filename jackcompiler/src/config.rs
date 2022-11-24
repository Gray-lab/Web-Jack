use std::{path::{PathBuf, Path}, fs};

#[derive(Debug)]
pub struct Config {
    pub file_paths: Vec<PathBuf>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("incorrect number of arguments");
        };
        
        let path = Path::new(&args[1]);
        let mut file_paths: Vec<PathBuf> = Vec::new(); 

        let metadata = match fs::metadata(path) {
            Ok(md) => md,
            // What if I want to return the actual error?
            Err(_) => {
                return Err("problem accesing filepath metadata")
            },
        };

        // Check if the path is a file or a directory
        if metadata.is_file() {

            let ext = match path.extension() {
                Some(ext) => ext,
                None => return Err("unable to access file extension")
            };

            // If file, check that it is a .jack file and add to config
            if ext == "jack" {
                file_paths.push(path.to_path_buf());
            } else {
                return Err("filename had incorrect extension")
            }

        } else {
            // If directory, add all .jack files to config
            let paths = match fs::read_dir(path) {
                Ok(paths) => paths,
                Err(_) => return Err("unable to access directory")
            };

            // Using unwrap here instead of handling the errors
            file_paths = paths
                .map(|path| path.unwrap().path())
                .filter(|path| { path.is_file() && path.extension().unwrap() == "jack" })
                .collect();
        }

        Ok(Config{ file_paths })
        
    }
}

