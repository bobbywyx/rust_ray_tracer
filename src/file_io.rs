use std::fs::File;

pub fn create_file() -> File {
    let mut file = File::create("./out.ppm").unwrap();
    return file;
}