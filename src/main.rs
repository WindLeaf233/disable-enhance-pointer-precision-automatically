// hide the console window
#![windows_subsystem = "windows"]

use std::{
    fs::{self, File},
    path::Path,
    thread::{self},
    time::Duration,
};

use sysinfo::System;
use winsafe::{
    co::{SPI, SPIF},
    prelude::user_Hwnd,
    SystemParametersInfo, HWND,
};

const CONFIG_PATH_FOLDER: &str = "C:\\deppa";
const CONFIG_PATH: &str = "C:\\deppa\\games.txt";

fn main() {
    // initialize
    let folder_path = Path::new(CONFIG_PATH_FOLDER);
    let config_path = Path::new(CONFIG_PATH);

    // create the config file if not exists
    if !folder_path.exists() {
        fs::create_dir(folder_path)
            .expect("failed to create config folder, consider creating it manually");
    }
    if !config_path.exists() {
        File::create(config_path)
            .expect("failed to create config file, consider creating it manually");
    }

    // read the game list from config file
    let content = fs::read_to_string(config_path).expect("failed to read the config file");
    let lines = content
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.to_string().to_lowercase())
        .collect::<Vec<String>>();

    // get mouse info
    unsafe {
        let mut mouse_params = [0u32; 3];
        SystemParametersInfo(SPI::GETMOUSE, 0, &mut mouse_params, SPIF::from_raw(0u32))
            .expect("failed to get mouse info from winapi, maybe you're not running Windows");

        let mut temp: String = String::new();
        loop {
            let (_, mut process_name) = get_active_window();
            process_name = process_name.to_lowercase();
            if process_name != temp.to_lowercase() && process_name != "" {
                // foreground process changed
                temp = process_name.clone();
                println!("foreground process changed: {}", process_name);

                if lines.contains(&process_name) {
                    // disable the enhance pointer precision for games
                    mouse_params[2] = 0;
                } else {
                    // enable the enhance pointer precision for other applications
                    mouse_params[2] = 1;
                }
                SystemParametersInfo(SPI::SETMOUSE, 0, &mut mouse_params, SPIF::SENDCHANGE)
                    .and_then(|_| {
                        println!("set mouse info: {} -> {}", process_name, mouse_params[2]);
                        println!();
                        Ok(())
                    })
                    .expect(&format!(
                        "failed to set mouse info: {} -> {}",
                        process_name, mouse_params[2]
                    ));
            }

            // loops take very high cpu usage, using `thread::sleep` to reduce
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

fn get_active_window() -> (u32, String) {
    let system = System::new_all();
    if let Some(hwnd) = <HWND as user_Hwnd>::GetForegroundWindow() {
        let (_, pid) = user_Hwnd::GetWindowThreadProcessId(&hwnd);
        // let name = user_Hwnd::GetWindowText(&hwnd).expect("failed to get process name");
        let name = system
            .processes()
            .iter()
            .find(|(ppid, _)| ppid.as_u32() == pid)
            .expect("failed to get process name")
            .1
            .name();
        return (pid, name.to_string());
    }

    // if api fails, return 0 and empty string
    (0, String::new())
}
