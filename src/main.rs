use clap::Parser;
use std::env;
use std::fs;
use std::io;
use std::path;

#[macro_use]
extern crate bitflags;

mod attributes;
mod filedata;
use filedata::*;

/// Lists all files in directory
#[derive(Parser)]
#[clap()]
struct Args {
    /// Display hidden files
    #[clap(short, long)]
    hidden: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let path = env::current_dir()?;

    print_dir(&path);
    print_header();

    let (mut dirs, mut files): (Vec<_>, Vec<_>) = fs::read_dir(path)?
        .filter_map(|result| result.ok())
        // read file meta data, map to (direntry,metadata)
        .filter_map(|entry| {
            entry
                .metadata()
                .map(|meta_data| FileData::new(entry, meta_data))
                .ok()
        })
        // filter hidden files/folders based on command line argument
        .filter(|entry| entry.is_hidden() == args.hidden)
        // split into dirs and folders
        .partition(|entry| entry.is_dir());

    // sort both by file name
    dirs.sort();
    files.sort();

    // print dirs first
    for entry in dirs {
        entry.print_entry()?;
    }
    // print files after
    for entry in files {
        entry.print_entry()?;
    }

    println!();
    println!();

    Ok(())
}

fn print_dir(path: &path::Path) {
    print!("\n\n    Directory: {}\n\n\n", path.display())
}

fn print_header() {
    println!("Mode                 LastWriteTime        Length Name");
    println!("----                 -------------        ------ ----");
}
