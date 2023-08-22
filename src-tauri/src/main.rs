// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use quartz_nbt::{io, NbtCompound, NbtList, NbtTag};
use quartz_nbt::io::{Flavor};
use tauri::{CustomMenuItem, Menu, Submenu};
use serde_json::{Map, Value};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn load(file: String) -> Result<NbtCompound, String> {
    let mut file = std::fs::File::open(file).unwrap();
    let nbt = io::read_nbt(&mut file, Flavor::GzCompressed).map_err(|e| e.to_string())?;
    return Ok(nbt.0);
}

#[tauri::command]
fn save_as_fun(file: String, nbt: Value) -> Result<(), ()> {
    fn to_tag(val: Value) -> NbtTag {
        match val {
            Value::Null => NbtTag::Byte(0),
            Value::Bool(x) => NbtTag::Byte(x as i8),
            Value::Number(x) => NbtTag::Long(x.as_i64().unwrap()),
            Value::String(st) => NbtTag::String(st),
            Value::Array(a) => {
                if a.len() == 2 && a.first().unwrap().is_string() {
                    return match a.first().unwrap().as_str().unwrap() {
                        "Byte" => NbtTag::Byte(a.last().unwrap().as_i64().unwrap() as i8),
                        "Short" => NbtTag::Short(a.last().unwrap().as_i64().unwrap() as i16),
                        "Int" => NbtTag::Int(a.last().unwrap().as_i64().unwrap() as i32),
                        "Long" => NbtTag::Long(a.last().unwrap().as_i64().unwrap()),
                        "Float" => NbtTag::Float(a.last().unwrap().as_f64().unwrap() as f32),
                        "Double" => NbtTag::Double(a.last().unwrap().as_f64().unwrap()),
                        "ByteArray" => NbtTag::ByteArray(a.last().unwrap().as_array().unwrap().iter().map(|x| x.as_i64().unwrap() as i8).collect()),
                        "IntArray" => NbtTag::IntArray(a.last().unwrap().as_array().unwrap().iter().map(|x| x.as_i64().unwrap() as i32).collect()),
                        "LongArray" => NbtTag::LongArray(a.last().unwrap().as_array().unwrap().iter().map(|x| x.as_i64().unwrap()).collect()),
                        "String" => NbtTag::String(a.last().unwrap().as_str().unwrap().to_string()),
                        "Compound" => NbtTag::Compound(to_compound(a.last().unwrap().as_object().unwrap().clone())),
                        "List" => {
                            let mut n = Vec::new();
                            for x in a.last().unwrap().as_array().unwrap() {
                                n.push(to_tag(x.clone()));
                            }
                            NbtTag::List(NbtList::from(n))
                        },
                        _ => NbtTag::List(NbtList::from(Vec::from([to_tag(a.first().unwrap().clone()), to_tag(a.last().unwrap().clone())])))
                    };
                }

                let mut n = Vec::new();
                for x in a {
                    n.push(to_tag(x.clone()));
                }
                return NbtTag::List(NbtList::from(n));
            }
            Value::Object(x) => NbtTag::Compound(to_compound(x))
        }
    }

    fn to_compound(val: Map<String, Value>) -> NbtCompound {
        let mut n = NbtCompound::new();

        for (k, v) in val {
            n.insert(k, to_tag(v))
        }

        n
    }

    let tag = to_compound(nbt.as_object().unwrap().clone());

    let mut file = std::fs::File::create(file).unwrap();
    io::write_nbt(&mut file, None, &tag, Flavor::GzCompressed).unwrap();
    return Ok(());
}

fn main() {
    let open = CustomMenuItem::new("open", "Open").accelerator("CmdOrControl+O");
    let reload = CustomMenuItem::new("reload", "Reload").accelerator("CmdOrControl+R");
    let save = CustomMenuItem::new("save", "Save").accelerator("CmdOrControl+S");
    let save_as = CustomMenuItem::new("save-as", "Save As").accelerator("CmdOrControl+Shift+S");
    let exit = CustomMenuItem::new("exit", "Exit").accelerator("CmdOrControl+Q");
    let file = Submenu::new("File", Menu::new().add_item(open).add_item(save).add_item(save_as).add_item(reload).add_item(exit));

    let gh = CustomMenuItem::new("gh", "GitHub");
    let help = Submenu::new("Help", Menu::new().add_item(gh));

    let menu = Menu::new().add_submenu(file).add_submenu(help);

    tauri::Builder::default()
        .menu(menu)
        .invoke_handler(tauri::generate_handler![load, save_as_fun])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
