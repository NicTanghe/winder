extern crate winpe;
use std::fs;
use std::io::Read;

fn main() {
    let file_path = "path/to/executable.exe";
    let mut file = fs::File::open(file_path).expect("Could not open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file");
    let image = winpe::Image::from_bytes(&buffer).expect("Could not parse PE image");

    match image.header.optional_header.subsystem {
        winpe::Subsystem::WindowsCui => println!("{} is a console application", file_path),
        winpe::Subsystem::WindowsGui => println!("{} is a GUI application", file_path),
        _ => println!("{} is not a Windows binary or its format not supported", file_path),
    }
}
