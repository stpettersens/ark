/*
    ark: Unix archiver (ar) implementation in Rust.
    Copyright (c) 2016 Sam Saint-Pettersen.

    Released under the MIT License.
*/

extern crate filetime;
use std::char;
use std::fs::File;
use self::filetime::FileTime;

struct ArEntry {
    file: String,
    modified: i32,
    owner: i32,
    group: i32,
    mode: i32,
    size: i32,
}

pub struct Ark;

impl Ark {
    fn pad_data(n: usize, data: &str) -> String {
        let mut padding = data.to_owned();
        for i in 0 .. (n - data.len()) {
            padding = format!("{}{}", padding, '0');
        }
        padding
    }

    fn write_ar_entries(archive: &str, entries: Vec<ArEntry>) {
        /*
         * COMMON AR FORMAT SPECIFICATION
         * (0) Global header
         * (a) Filename in ASCII [0:16]
         * (b) File modification timestamp (Decimal) [16:12]
         * (c) Owner ID (Decimal) [28:6]
         * (d) Group ID (Decimal) [34:6]
         * (e) File mode (Octal) [40:8]
         * (f) File size in bytes (Decimal) [48:10]
         * (g) Magic number ("0x60 0x0A") [58:2]
        */
        let mut ar = File::create(archive).unwrap();
        let header = format!("!<arch>{}", char::from_u32(0x0A).unwrap()); // 0
        for i in 0 .. entries.len() {

        }
    }

    pub fn create_archive(archive: &str, filenames: Vec<&str>) {
        let mut entries: Vec<ArEntry> = Vec::new();
        for i in 0 .. filenames.len() {
            entries.push(ArEntry { 
                file: filenames[i], 
                modified: 0, 
                owner: 0,
                group: 0,
                mode: 0,
                size: 0
            })
        }
        Ark::write_ar_entries(archive, filenames);
    }
}
