fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cc")
        .file("src/concat.cc")
        .std("c++20")
        .compile("cxxdemo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/blobstore.cc");
    println!("cargo:rerun-if-changed=include/blobstore.h");
}