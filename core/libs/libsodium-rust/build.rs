// Downloads and builds libsodium for rustc to consume
extern crate reqwest;

use std::fs::File;
use std::process::Command;
use std::env;


fn download() {
    let url = "https://download.libsodium.org/libsodium/releases/libsodium-1.0.16.tar.gz";
    let destination = "./libsodium-1.0.16.tar.gz";
    println!("Downloading {}", url);
    println!("Saving to {}", destination);
    let mut response = reqwest::get(url).unwrap();
    let mut file = File::create(destination).unwrap();
    response.copy_to(&mut file);
}

fn unpack() {

}

fn make() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();
    /*
     * ./configure && ./make && ./make install DESTDIR=./
     */
    Command::new("configure").args(&["--target", &target])
                       .status().unwrap();
    Command::new("make").status().unwrap();
    Command::new("make").arg(&"install")
                       .arg(&format!("DESTDIR={}", env::current_dir().unwrap().to_str().expect("Failed to get current dir")))
                       .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=sodium");
}

fn main() {
    download();
}