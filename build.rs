fn main() {
    // Pull from sys, re-emit under our own namespace
    let redist_file = std::env::var("DEP_STEAM_API_REDIST_FILE")
        .expect("steamworks-sys must emit redist_file metadata");
    println!("cargo::metadata=redist_file={}", redist_file);
    println!("cargo::rerun-if-changed=build.rs");
}
