extern crate cc;

fn main() {
    cc::Build::new()
        .file("mcmf.cpp")
        .include(".")
        .opt_level(3)
        .cpp(true)
        .compile("libwasserstein.a");
    println!("cargo:rerun-if-changed=lemon");
}
