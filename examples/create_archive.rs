extern crate ark;
use ark::Ark;

fn main() {
    Ark::create_archive("archive.ar", vec!["Cargo.toml",".gitignore"]);
}
