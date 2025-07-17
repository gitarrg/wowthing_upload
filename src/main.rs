// standard libraries
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str;
use std::time;

// third party
use debounce::EventDebouncer;
use notify;
use notify::{RecursiveMode, Watcher};
use reqwest;
use serde::Deserialize;
extern crate chrono;
use chrono::Local;


/*******************************************************************************
 * Config
**/

// Config File
#[derive(Deserialize)]
struct Config {
    api_key: String,
    host: String,
    files: Vec<String>,
}

fn load_config(path: &PathBuf) -> Result<Config, String> {
    let contents: String = fs::read_to_string(path)
        .map_err(|err| format!("Unable to read config file: {path:?}. {}", err))?;

        let config: Config = toml::from_str(&contents)
        .map_err(|err| format!("Unable to parse config file: {path:?}. {}", err))?;

        return Ok(config);
}

/*******************************************************************************
 * Upload
**/

fn upload_file(path: &String, config: &Config) {

    let now = Local::now().format("%Y-%m-%dT%H:%M:%S");
    println!("[{now}] Uploading {path:?}");

    let contents: String = fs::read_to_string(path).unwrap();
    let mut data = HashMap::new();
    data.insert("ApiKey", String::from(&config.api_key));
    data.insert("LuaFile", contents);

    let host: &String = &config.host;
    let url: String = format!("{host}/api/upload/");

    let client = reqwest::blocking::Client::new();
    let res = client.post(url).json(&data).send();

    match res {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error uploading file: {e}");
            return;
        }
    }
}

/*******************************************************************************
 * Main
**/

fn main() {

    let now = Local::now().format("%Y-%m-%dT%H:%M:%S");
    println!("[{now}] hello!");


    // Construct path to config
    let exe_path = std::env::current_exe().expect("Failed to get current exe path");
    let exe_dir = exe_path.parent().expect("Executable must be in a directory");
    let config_path = exe_dir.join("config.toml");
    let config = match load_config(&config_path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("[{now}] Unable to load config file. Error: {e}");
            let _ = std::io::stdin().read_line(&mut String::new());
            return;
        }
    };

    // let api_key = config.api_key;
    // for path in &config.files {
    //     if Path::new(&path).exists() {
    //         upload_file(&path, &config);
    //     }
    // }

    // Add files to the watcher
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx).unwrap();

    for path in &config.files {
        match watcher.watch(Path::new(path), RecursiveMode::NonRecursive) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("[{now}] Unable to watch file. Error: {e}");
                continue;
            }
        };
    }

    let delay = time::Duration::from_secs(3);
    let debouncer = EventDebouncer::new(delay, move |path: String| {
        upload_file(&path, &config);
    });

    // main loop
    for res in rx {
        match res {
            Ok(event) => {
                // on windows we're only getting 3 events:
                // [Remove(any), Modify(Name(to)), Modify(Any)]
                // linux triggers a few more but well.. we gotta work with what we have
                if !event.kind.is_modify() {
                    continue;
                }

                for path in event.paths {
                    debouncer.put(path.to_string_lossy().to_string());
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

}
