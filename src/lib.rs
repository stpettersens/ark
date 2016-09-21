/*
    ark: Unix archiver (ar) implementation in Rust.
    Copyright (c) 2016 Sam Saint-Pettersen.

    Released under the MIT License.
*/

extern crate filetime;
use std::char;
use std::fs::File;
//use self::filetime::FileTime;

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
        let ar = File::create(archive).unwrap();
        let header = format!("!<arch>{}", char::from_u32(0x0A).unwrap()); // (0)
        for i in 0 .. entries.len() {
            let filename = Ark::pad_data(16, &format!("{}/", entries[i].file)); // (a)
            let modified = Ark::pad_data(12, &format!("{}", entries[i].modified)); // (b)
            let owner = Ark::pad_data(6, &format!("{}", entries[i].owner)); // (c)
            let group = Ark::pad_data(6, &format!("{}", entries[i].group)); // (d)
            let mode = Ark::pad_data(8, &format!("{}", entries[i].mode)); // (e)
            let size = Ark::pad_data(10, &format!("{}", entries[i].size)); // (f)
            let magic = &format!("{}{}", char::from_u32(0x60).unwrap(), char::from_u32(0x0A).unwrap()); // (g)
            let mut input = File::open(&format!("{}", entries[i].file)).unwrap();
            let mut contents = String::new();
            let _ = input.read_to_string(&mut contents);
            let data = format!("{}{}{}{}{}{}{}{}", filename, modified, owner, group, mode, size, magic, contents);
            let hd = format!("{}{}", header, data);
            let _ = ar.write_all(hd.as_bytes());
        }
    }

    pub fn create_archive(archive: &str, filenames: Vec<&str>) {
        let mut entries: Vec<ArEntry> = Vec::new();
        for i in 0 .. filenames.len() {
            entries.push(ArEntry {
                file: filenames[i].to_owned(),
                modified: 0,
                owner: 0,
                group: 0,
                mode: 0,
                size: 0
            })
        }
        Ark::write_ar_entries(archive, entries);
    }
}

#[cfg(test)]
#[test]
fn create_archive() {
    Ark::create_archive("archive.ar", vec!["Cargo.toml", ".gitignore"]);
}
