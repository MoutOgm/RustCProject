use std::fs;
fn main() {
    
    let paths = fs::read_dir("./").unwrap();
    let uncrate = ["src".to_string(), "target".to_string()];
    for path in paths {
        let path_dir = path.unwrap();
        if ! path_dir.file_type().unwrap().is_dir() || uncrate.contains(&path_dir.file_name().into_string().unwrap()) {
            continue;
        }
        let crate_dir = path_dir.file_name().into_string().unwrap();
        cbindgen::Builder::new()
            .with_crate(crate_dir.to_string())
            .with_namespaces(&["rc".to_string(), crate_dir.to_string()])
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file("target/release/lib/".to_owned() + &crate_dir + &".hpp");
    }
}