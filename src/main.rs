use std::{
    collections::HashMap,
    fs::{self, write, File},
    ops::Range,
    path::Path,
};

use clap::Parser;
use serde_json::{json, Map, Value};

#[derive(Parser, Debug)]
#[clap(name = "Data combiner")]
#[clap(about="Generate a .dat file containing all the files and a .json map file", long_about = None)]
#[clap(author, version)]
struct Args {
    /// Source directory
    #[clap(name = "src", value_parser)]
    src_arg: Option<String>,

    /// Source directory
    #[clap(short, long = "src", value_parser, about)]
    src_opt: Option<String>,

    /// Out directory
    #[clap(short, long, value_parser)]
    out: Option<String>,
}

fn main() {
    let args = Args::parse();

    let src_dir = args
        .src_opt
        .unwrap_or(args.src_arg.unwrap_or(".".to_string()));

    let out_dir = args.out.unwrap_or(".".to_string());

    println!("src: '{src_dir}', out: '{out_dir}'");

    let mut json_map = Map::new();

    fs::read_dir(src_dir)
        .unwrap()
        .map(Result::unwrap)
        .for_each(|file| {
            let path = file
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();

            let name = path.split_at(path.len() - 4).0.to_string();
            let name_length = name.chars().count();

            println!("_____________________");
            println!("name: {name}");

            // //go down the tree until encountering an unassigned node
            // let mut sub_json_down = &json_map;
            // let mut down_until_ind = name_length;
            // for (ind_down, char) in name.chars().enumerate() {
            //     if !sub_json_down.contains_key(&char.to_string()) {
            //         down_until_ind = ind_down;
            //         break;
            //     }
            //     println!("down to {char}");
            //     sub_json_down = sub_json_down
            //         .get(&char.to_string())
            //         .unwrap()
            //         .as_object()
            //         .unwrap();
            // }

            // println!("{name} go down until {down_until_ind}");

            //build from bottom
            let mut sub_json = json!([1, 2]);
            for ind in 1..=name_length {
                let addr = substring(&name, 0..((name_length - ind) as i32));

                let existing_sub_json_wrapped =
                    get_sub_json_value(&Value::Object(json_map.clone()), addr);

                let char_wrapped = addr.chars().last();
                if let Some(existing_sub_json) = existing_sub_json_wrapped {
                    println!("addr: {addr} exists");
                    let mut from_iter: Map<String, Value>;
                    if let Some(char) = char_wrapped {
                        from_iter = existing_sub_json.as_object().unwrap().clone();
                        from_iter.insert(char.to_string(), sub_json);
                    } else {
                        from_iter = existing_sub_json.as_object().unwrap().clone();
                        for (key, val) in sub_json.as_object().unwrap().iter() {
                            from_iter.insert(key.to_string(), val.clone());
                        }
                    }
                    sub_json = Value::Object(from_iter);
                } else {
                    sub_json = Value::Object(Map::from_iter([(
                        char_wrapped.unwrap().to_string(),
                        sub_json,
                    )]));
                }
            }
            let sub_json_up_as_map = sub_json.as_object().unwrap();
            json_map = sub_json_up_as_map.clone();

            // //build from bottom
            // let mut sub_json_up = json!([1, 2]);
            // for (ind, char) in name.chars().rev().enumerate() {
            //     if ind == name_length - down_until_ind {
            //         //use already existing struct

            //         let addr = substring(&name, 0..((name_length - ind - 0) as i32));
            //         println!("addr {addr}, {}", ((name_length - ind - 0) as i32));

            //         let existing =
            //             get_sub_json_value(&Value::Object(json_map.clone()), addr).unwrap();
            //         let mut from_iter = existing.as_object().unwrap().clone();
            //         from_iter.insert(char.to_string(), sub_json_up);
            //         sub_json_up = Value::Object(from_iter);
            //     } else {
            //         sub_json_up = Value::Object(Map::from_iter([(char.to_string(), sub_json_up)]));
            //     }
            //     println!("{}", serde_json::to_string(&sub_json_up).unwrap());
            // }
            // let sub_json_up_as_map = sub_json_up.as_object().unwrap();
            // json_map = sub_json_up_as_map.clone();
        });

    let json_map_str = serde_json::to_string(&json_map).unwrap();

    _ = write(Path::join(Path::new(&out_dir), "out.json"), json_map_str);

    println!("done");
}

fn substring(str: &str, range: Range<i32>) -> &str {
    if range.start == range.end {
        return "";
    }

    let end: usize;

    if range.end <= 0 {
        end = ((str.chars().count() as i32) + range.end) as usize;
    } else {
        end = range.end as usize;
    }

    let start_part = str.split_at(range.start as usize).1;
    return start_part.split_at(end).0;
}

fn get_sub_json_value(json_map: &Value, addr: &str) -> Option<Value> {
    let mut current = json_map;

    println!("addddr {addr}");
    if addr.is_empty() {
        return Some(json_map.clone());
    }

    for char in addr.chars() {
        if let Some(curr) = current.as_object()?.get(&char.to_string()) {
            current = curr
        } else {
            return None;
        }
    }

    return Some(current.clone());
}
