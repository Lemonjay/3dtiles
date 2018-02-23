extern crate cc;
use std::fs;
//use std::process::Command;

fn main() {
    fs::create_dir_all("target/debug").unwrap();
    fs::create_dir_all("target/release").unwrap();
    
    if cfg!(windows) {
        cc::Build::new()
            .cpp(true)
            .warnings(false)
            .define("WIN32",None)
            .include("./src")
            .include("./src/osg")
            .file("./src/tileset.cpp")
            .file("./src/shp23dtile.cpp")
            .file("./src/osgb23dtile.cpp")
            .compile("3dtile");
        // -------------
        println!("cargo:rustc-link-search=native=./lib");
        // -------------
        println!("cargo:rustc-link-lib=gdal_i");
        println!("cargo:rustc-link-lib=osg");
        println!("cargo:rustc-link-lib=osgDB");
        println!("cargo:rustc-link-lib=osgUtil");
    } else {
        println!("cargo:rustc-link-search=native=.");
    }
}
