use cargo_metadata::MetadataCommand;

pub fn print_metadata() {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let meta = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .current_dir(&path)
        .exec()
        .expect("Unable to find cargo.toml");

    let root = meta.root_package().expect("No root package");

    let name: &str = root.metadata["default"]["name"].as_str().unwrap();
    println!("cargo::rustc-env=NAME={}", name);
    let short_name: &str = root.metadata["default"]["short-name"].as_str().unwrap();
    println!("cargo::rustc-env=SHORT_NAME={}", short_name);
}

fn main() {
    print_metadata()
}
