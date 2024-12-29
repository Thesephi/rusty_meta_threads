use std::collections::HashMap;
use std::{env, fs};

pub fn read_dot_env() -> HashMap<String, String> {
    let the_path = env::current_dir().unwrap().join(".env");
    let dot_env_data =
        fs::read_to_string(&the_path).expect(&format!("Unable to read {:?}", the_path));
    // @TODO if .env file is not there, read directly from std::env
    // @TODO strip end-of-line comments
    let mut ret_val = HashMap::new();
    for line in dot_env_data.lines() {
        if !line.starts_with("#") && !line.is_empty() {
            let pair = line.split("=").collect::<Vec<&str>>();
            ret_val.insert(String::from(pair[0]), String::from(pair[1]));
        }
    }
    ret_val
}
