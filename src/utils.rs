use log::debug;
use std::collections::HashMap;
use std::{env, fs};

pub fn read_dot_env() -> HashMap<String, String> {
    let mut ret_val = HashMap::<String, String>::new();
    if let Ok(cur_dir) = env::current_dir() {
        let the_path = cur_dir.join(".env");
        debug!("attempting to load env from {:?}", the_path);
        if let Ok(dot_env_data) = fs::read_to_string(&the_path) {
            for line in dot_env_data.lines() {
                if let Some((pair_str, _comment)) = line.split_once("#") {
                    // line has comment
                    process_dot_env_line(pair_str, &mut ret_val);
                } else {
                    // line does not have comment
                    process_dot_env_line(line, &mut ret_val);
                }
            }
        }
    }
    // as per convention, we overwrite .env with direct env vars
    for (key, val) in env::vars() {
        ret_val.insert(key, val);
    }
    ret_val
}

fn process_dot_env_line(line: &str, hm: &mut HashMap<String, String>) {
    if let Some((p1, p2)) = line.split_once("=") {
        hm.insert(String::from(p1), String::from(p2));
        debug!("read env `{:?}`", p1);
    }
}
