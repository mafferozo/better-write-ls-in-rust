use crate::attributes::*;
use chrono::{DateTime, Local};
use std::cmp::Ordering;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::os::windows::prelude::*;

/// a pair containing a DirEntry and its own MetaData
pub struct FileData(fs::DirEntry, fs::Metadata);

impl FileData {
    pub fn new(entry: fs::DirEntry, meta: fs::Metadata) -> Self {
        FileData(entry, meta)
    }

    pub fn get_flags(&self) -> Attributes {
        Attributes::from_bits_truncate(self.1.file_attributes())
    }

    pub fn is_hidden(&self) -> bool {
        self.get_flags().contains(Attributes::HIDDEN)
    }

    pub fn is_dir(&self) -> bool {
        self.1.is_dir()
    }

    pub fn print_entry(&self) -> io::Result<()> {
        let (entry, meta_data) = (&self.0, &self.1);

        // format modified date time
        let dt: DateTime<Local> = meta_data.modified()?.into();
        let dt_format = dt.format("%_m/%_d/%Y  %_I:%M %p");

        println!(
            "{}        {}        {:7} {}",
            self.get_flags(),
            dt_format,
            meta_data.len(),
            entry.file_name().to_string_lossy()
        );

        Ok(())
    }

    // used for ordering impl
    fn file_name(&self) -> OsString {
        self.0.file_name()
    }
}

impl Ord for FileData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file_name().cmp(&other.file_name())
    }
}

impl PartialOrd for FileData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.file_name().cmp(&other.file_name()))
    }
}

impl PartialEq for FileData {
    fn eq(&self, other: &Self) -> bool {
        self.file_name() == other.file_name()
    }
}

impl Eq for FileData {}
