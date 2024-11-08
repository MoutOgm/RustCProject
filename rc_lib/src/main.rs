use std::fs;
fn main() {
    let paths = fs::read_dir("./").unwrap();
    let uncrate = ["src".to_string(), "target".to_string()];
    for path in paths {
        let path_dir = path.unwrap();
        if !path_dir.file_type().unwrap().is_dir()
            || uncrate.contains(&path_dir.file_name().into_string().unwrap())
        {
            continue;
        }
        let crate_dir = path_dir.file_name().into_string().unwrap();
        cbindgen::Builder::new()
            .with_crate(crate_dir.to_string())
            .with_language(cbindgen::Language::Cxx)
            .with_namespaces(&["rc".to_string(), crate_dir.to_string()])
            .with_parse_expand(&[crate_dir.to_string()])
            .with_no_includes()
            .with_item_prefix("class")
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(format!("target/release/lib/{}.hpp", crate_dir));
    }
}