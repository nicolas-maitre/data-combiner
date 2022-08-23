use std::{
    fs::{self, write},
    ops::Range,
    path::Path,
};

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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

#[derive(Serialize, Deserialize)]
struct FileInfo {
    n: String,
    p: usize,
    l: usize,
}

fn main() {
    let args = Args::parse();

    let src_dir = args
        .src_opt
        .unwrap_or(args.src_arg.unwrap_or(".".to_string()));

    let out_dir = args.out.unwrap_or(".".to_string());

    println!("src: '{src_dir}', out: '{out_dir}'");

    let file_infos = fs::read_dir(src_dir)
        .unwrap()
        .map(|res| {
            let path = res.unwrap().path();
            let full_name = path.file_name().unwrap().to_str().unwrap();
            let name = substring(full_name, 0..-4);

            let pos = 0;
            let length = 1;
            return FileInfo {
                n: name.to_string(),
                p: pos,
                l: length,
            };
        })
        .collect::<Vec<_>>();

    let file_infos_str = serde_json::to_string(&file_infos).unwrap();

    _ = write(Path::join(Path::new(&out_dir), "out.json"), file_infos_str);

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
