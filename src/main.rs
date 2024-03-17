use std::{
    fs::{self, File},
    path::Path,
};

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
    if folder_path.exists() {
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
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // get mouse info
    unsafe {
        let mut mouse_params = [0u32; 3];
        SystemParametersInfo(SPI::GETMOUSE, 0, &mut mouse_params, SPIF::from_raw(0u32))
            .expect("failed to get mouse info from winapi, maybe you're not running Windows");

        loop {
            let (_, process_name) = get_active_window();
            if lines.contains(&process_name) {
                // disable the enhance pointer precision for games
                mouse_params[2] = 0;
            } else {
                // enable the enhance pointer precision for other applications
                mouse_params[2] = 1;
            }
            SystemParametersInfo(SPI::SETMOUSE, 0, &mut mouse_params, SPIF::SENDCHANGE).expect(
                &format!(
                    "failed to set mouse info: ({}) {} -> {}",
                    process_name, !mouse_params[2], mouse_params[2]
                ),
            );
        }
    }
}

fn get_active_window() -> (u32, String) {
    if let Some(hwnd) = <HWND as user_Hwnd>::GetForegroundWindow() {
        let (_, pid) = user_Hwnd::GetWindowThreadProcessId(&hwnd);
        let name = user_Hwnd::GetWindowText(&hwnd).expect("failed to get process name");
        return (pid, name);
    }

    // if api fails, return 0 and empty string
    (0, String::new())
}
