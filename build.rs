fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-search=native={}", std::env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=tree-sitter-rust");
    println!("cargo:rustc-link-lib=static=tree-sitter-python");
}
