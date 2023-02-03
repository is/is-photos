use regex::Regex;
use walkdir::WalkDir;


fn main() {
    let r0 = Regex::new(r"(\d{8}__KEEP)").unwrap();
    let dirs: Vec<String> = WalkDir::new(".")
        .max_depth(1).min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .map(|e| e.path().file_name().unwrap().to_str().unwrap().to_string())
        .collect();

    for dir_name in &dirs {
        println!("-- {}", dir_name);
        if let Some(_) = r0.captures(dir_name) {
            let pair_name = dir_name.to_string();
            let pair_name = pair_name.replace("__KEEP_", "__");
            if dirs.contains(&pair_name) {
                println!("{}", pair_name);
            }
        }
    }
}
