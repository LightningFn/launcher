extern crate winapi;

use std::path::Path;
use std::env;
use std::fs;
use std::process::exit;
use std::process::Command;
use std::ops::Not;
use std::ptr::null_mut as NULL;

use colored::Colorize;
use serde_derive::Deserialize;
use toml;
use winapi::um::winuser;

#[derive(Deserialize)]
struct Lexec {
  version: i32,
  instance: Instance,
}

#[derive(Deserialize)]
struct Instance {
  name: String,
  path: String,
  args: Vec<String>,
  username: String,
}

fn errmsgbox(message: String) {
  unsafe {
    let msg: Vec<u16> = message.encode_utf16().collect();
    let title: Vec<u16> = "Error\0".encode_utf16().collect();
    winuser::MessageBoxW(NULL(), msg.as_ptr(), title.as_ptr(), winuser::MB_OK | winuser::MB_ICONSTOP);
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  

  println!("{}", format!(r#"
  / $$       /$$           /$$         /$$               /$$               
  | $$      |__/          | $$        | $$              |__/ 
  | $$       /$$  /$$$$$$ | $$$$$$$  /$$$$$$   /$$$$$$$  /$$ /$$$$$$$   /$$$$$$ 
  | $$      | $$ /$$__  $$| $$__  $$|_  $$_/  | $$__  $$| $$| $$__  $$ /$$__  $$
  | $$      | $$| $$  \ $$| $$  \ $$  | $$    | $$  \ $$| $$| $$  \ $$| $$  \ $$
  | $$      | $$| $$  | $$| $$  | $$  | $$ /$$| $$  | $$| $$| $$  | $$| $$  | $$
  | $$$$$$$$| $$|  $$$$$$$| $$  | $$  |  $$$$/| $$  | $$| $$| $$  | $$|  $$$$$$$
  |________/|__/ \____  $$|__/  |__/   \___/  |__/  |__/|__/|__/  |__/ \____  $$
                 /$$  \ $$                                             /$$  \ $$
                |  $$$$$$/                                            |  $$$$$$/
                 \______/                                              \______/
  "#).truecolor(194, 122, 228));

  if Path::new(&args[1]).is_file().not() {
    errmsgbox(format!("Error: File `{}` not fount.\0", args[1]));
    exit(1);
  }

  let content = match fs::read_to_string(&args[1]) {
    Ok(c) => c,
    Err(_) => {
      errmsgbox(format!("Error: Could not read file `{}`.\0", &args[1]));
      exit(1);
    }
  };

  let lexec: Lexec = match toml::from_str(&content) {
    Ok(d) => d,
    Err(_) => {
      errmsgbox(format!("Error: Unable to load data from file `{}`.\0", args[1]));
      exit(1);
    }
  };

  if lexec.version == 1 {
    println!("Launching {}", lexec.instance.name);
    if Path::new(&lexec.instance.path).is_file().not() {
      errmsgbox(format!("Error: File `{}` not found.\0", lexec.instance.path));
      exit(1);
    }
    let mut args: Vec<String> = Vec::new();
    for i in 0..lexec.instance.args.len() {
      // let username: &str = &*lexec.instance.username;
      let a = format!("{}", lexec.instance.args[i].replace("!USERNAME!", &lexec.instance.username));

      // println!("{}", lexec.instance.args[i].replace("!USERNAME!", &lexec.instance.name));
      println!("{}", a);
      // args.push(a);
      args.push(format!("{}", lexec.instance.args[i].replace("!USERNAME!", &lexec.instance.username)))
      // let a = lexec.instance.args[i].replace("!USERNAME!", *&lexec.instance.username);
      // args.push(&lexec.instance.args[i].replace("!USERNAME!", username));
    }
    Command::new(lexec.instance.path.replace("\\", "\\\\")).args(args).spawn().expect("error");
  }
}