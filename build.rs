use std::process::Command;

fn store_git_commit_hash() {
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}

fn generate_c_header() {
  let output_path = "target/api.h";
  let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
  let mut config = cbindgen::Config::default();
  config.language = cbindgen::Language::C;
  config.no_includes = true;

  cbindgen::generate_with_config(crate_dir, config)
    .unwrap()
    .write_to_file(output_path);
}

fn main() {
  store_git_commit_hash();
  generate_c_header();
}