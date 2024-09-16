use std::io::{self, Write};
use std::path::{Path, PathBuf};
use prost_build;

fn main() {
    let proto_file = prompt("Enter proto file name: ");
    let output_dir = prompt("Enter the directory where you want the output to be generated: ");

    let proto_path = Path::new(&proto_file);
    let output_path = PathBuf::from(&output_dir);

    if proto_path.exists() {
        if let Err(e) = compile_proto(proto_path, output_path) {
            eprintln!("Failed to compile proto file: {}", e);
        } else {
            println!("Successfully compiled the proto file!");
        }
    } else {
        eprintln!("The proto file '{}' does not exist.", proto_file);
    }
}

fn compile_proto(proto_path: &Path, output_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    prost_build::Config::new()
        .out_dir(&output_path)
        .compile_protos(&[proto_path], &["."])?;

    Ok(())
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}
