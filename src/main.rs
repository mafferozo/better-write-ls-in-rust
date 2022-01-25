use chrono::{DateTime, Local};
use clap::Parser;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::os::windows::prelude::*;
use std::path;

#[macro_use]
extern crate bitflags;

mod attributes;
use attributes::*;

/// Lists all files in directory
#[derive(Parser)]
#[clap()]
struct Args {
    /// Display hidden files
    #[clap(short, long)]
    hidden: bool,
}

/// a pair containing a DirEntry and its own MetaData
struct FileData(fs::DirEntry, fs::Metadata);

impl FileData {
    fn get_flags(&self) -> Attributes {
        Attributes::from_bits_truncate(self.1.file_attributes())
    }

    fn is_hidden(&self) -> bool {
        self.get_flags().contains(Attributes::HIDDEN)
    }

    fn is_dir(&self) -> bool {
        self.1.is_dir()
    }

    fn file_name(&self) -> OsString {
        self.0.file_name()
    }

    fn print_entry(&self) -> io::Result<()> {
        let (dir_entry, meta_data) = (&self.0, &self.1);
        let flags = self.get_flags();
        let dt: DateTime<Local> = meta_data.modified()?.into();
        let dt_format = dt.format("%_m/%_d/%Y  %_I:%M %p");

        println!(
            "{}        {}        {:7} {}",
            flags,
            dt_format,
            meta_data.len(),
            dir_entry.file_name().to_string_lossy()
        );

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let path = env::current_dir()?;

    print_dir(&path);
    print_header();

    let (mut dirs, mut files): (Vec<_>, Vec<_>) = fs::read_dir(path)?
        .filter_map(|x| x.ok())
        // read file meta data, map to (direntry,metadata)
        .filter_map(|x| x.metadata().map(|data| FileData(x, data)).ok())
        // filter hidden files/folders based on command line argument
        .filter(|file_data| args.hidden == file_data.is_hidden())
        .partition(|e| e.is_dir());

    dirs.sort_by(|a, b| a.file_name().partial_cmp(&b.file_name()).unwrap());
    files.sort_by(|a, b| a.file_name().partial_cmp(&b.file_name()).unwrap());

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
