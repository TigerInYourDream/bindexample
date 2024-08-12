extern crate bindgen;

use std::path::PathBuf;

use bindgen::CargoCallbacks;


fn main() {

    println!("cargo:rustc-link-search=native=./lib");
    println!("cargo:rustc-link-lib=dylib=MyClassWrapper");
    println!("cargo:rustc-link-lib=dylib=MyClass");

    // 生成 Rust 绑定
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // 将生成的绑定写入 src/bindings.rs 文件
    let out_path = PathBuf::from("./src/");   
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}