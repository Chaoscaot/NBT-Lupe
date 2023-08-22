// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use quartz_nbt::{io, NbtCompound};
use quartz_nbt::io::{Flavor};
use tauri::{CustomMenuItem, Menu, Submenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn load(file: String) -> Result<NbtCompound, String> {
    let mut file = std::fs::File::open(file).unwrap();
    let nbt = io::read_nbt(&mut file, Flavor::GzCompressed).map_err(|e| e.to_string())?;
    return Ok(nbt.0);
}

fn main() {
    let open = CustomMenuItem::new("open", "Open").accelerator("CmdOrControl+O");
    let reload = CustomMenuItem::new("reload", "Reload").accelerator("CmdOrControl+R");
    let save = CustomMenuItem::new("save", "Save").accelerator("CmdOrControl+S");
    let save_as = CustomMenuItem::new("save-as", "Save As").accelerator("CmdOrControl+Shift+S");
    let exit = CustomMenuItem::new("exit", "Exit").accelerator("CmdOrControl+Q");
    let file = Submenu::new("File", Menu::new().add_item(open).add_item(save).add_item(save_as).add_item(exit));

    let menu = Menu::new().add_submenu(file);

    tauri::Builder::default()
        .menu(menu)
        .invoke_handler(tauri::generate_handler![load])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
