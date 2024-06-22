use std::fs::File;

pub fn create_file(name: String) -> File {
    let file_path = format!("./{}.ppm", name);
    let mut file = File::create(file_path).unwrap();
    return file;
}
