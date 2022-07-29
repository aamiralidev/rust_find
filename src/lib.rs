use clap::ArgMatches;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn run(config: &Config) -> Result<(), &'static str> {
    // 1. parse patterns
    let v_pats = config.parse_patterns()?;

    // 2. get directories
    let v_dirs = config.parse_dirs()?;

    // 3. parse optional arguments
    let mut output = config.parse_output();

    let size = config.parse_size();

    // 4. get files and output
    let mut matched_files = Vec::with_capacity(v_dirs.len());
    for dir in v_dirs.iter() {
        get_matched_files(&mut matched_files, dir, &v_pats[..], size);

        // print or write
        if let Some(sv) = display(&matched_files, &mut output) {
            for s in sv {
                println!("{}", s);
            }
        };
        matched_files.clear();
    }

    Ok(())
}

pub struct Config<'a> {
    pub dirs: Vec<&'a str>,
    pub patterns: Vec<&'a str>,
    pub output: Option<&'a str>,
    pub size: Option<&'a str>,
}

impl<'a> Config<'a> {
    pub fn from_args(args: &'a ArgMatches) -> Self {
        println!("{:?}", args);
        Config {
            dirs: args.values_of("dirs").unwrap().collect(),
            patterns: args.values_of("patterns").unwrap().collect(),
            output: args.value_of("output"),
            size: args.value_of("size"),
        }
    }
}
impl<'a> Config<'a> {
    pub fn parse_patterns(&self) -> Result<Vec<Regex>, &'static str> {
        let mut reg_vec: Vec<Regex> = Vec::new();
        for x in self.patterns.iter() {
            match Regex::new(x) {
                Ok(y) => reg_vec.push(y),
                Err(_) => eprintln!("Oh no we have Regex error"),
            };
        }
        Ok(reg_vec)
    }

    pub fn parse_dirs(&self) -> Result<Vec<PathBuf>, &'static str> {
        let mut reg_vec: Vec<PathBuf> = Vec::new();
        for x in self.dirs.iter() {
            reg_vec.push(PathBuf::from(x));
        }
        Ok(reg_vec)
    }

    pub fn parse_output(&self) -> Option<File> {
        self.output.as_ref().map(|y| File::create(y).unwrap())
    }

    pub fn parse_size(&self) -> Option<u64> {
        match self.size {
            Some(val) => {
                let my_int: u64 = val.parse().unwrap();
                Some(my_int)
            }
            None => None,
        }
    }
}
pub fn get_matched_files(files: &mut Vec<MyFile>, dir: &Path, v_pats: &[Regex], size: Option<u64>) {
    // call get_matched_files() in itself if the given directory `dir` contains a sub-directory
    if dir.is_file() {
        return;
    }
    for rd in std::fs::read_dir(dir).unwrap() {
        // To get path from read_dir() result
        let pth = rd.unwrap().path();
        // To check whether a path is a directory
        if pth.is_dir() {
            get_matched_files(files, &pth, v_pats, size);
        } else {
            for regx in v_pats.iter() {
                if regx.is_match(pth.file_name().unwrap().to_str().unwrap()) {
                    if pth.metadata().unwrap().len() > size.unwrap_or(0) {
                        files.push(MyFile::from_path(&pth).unwrap())
                    }
                    break;
                }
            }
        }
    }
}

pub struct MyFile {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
}
impl MyFile {
    /// Instantiate a MyFile struct from the path of a file.
    pub fn from_path(path: &Path) -> Result<Self, &'static str> {
        if path.exists() {
            if path.is_file() {
                Ok(MyFile {
                    name: path.file_name().unwrap().to_str().unwrap().to_string(),
                    path: path.to_str().unwrap().to_string(),
                    size_bytes: path.metadata().unwrap().len(),
                })
            } else {
                Err("Path is a dir")
            }
        } else {
            Err("Path doesn't exists")
        }
    }
}

pub fn display(files: &[MyFile], output: &mut Option<File>) -> Option<Vec<String>> {
    let mut res_vec: Vec<String> = vec![];
    for file in files {
        res_vec.push(file.path.clone())
    }
    if let Some(out_file) = output {
        for elem in res_vec {
            if let Err(err) = out_file.write_all((elem + "\n").as_bytes()) {
                println!("DISPLAY: Error writing file {}", err);
            }
        }
        None
    } else {
        Some(res_vec)
    }
}
