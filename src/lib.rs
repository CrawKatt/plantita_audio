use std::process::Command;
use std::fs::File;
use std::io::{Read, Write};

/// Convertir el archivo recibido a .mp3
pub fn convert_to_mp3(input_path: &str, output_path: &str) -> std::io::Result<String> {
    // Open the input file
    let mut input_file = File::open(input_path)?;

    // Necesario para que el archivo de salida tenga la extensión .mp3 y sea reconocido como un archivo en lugar de un directorio
    let output_path = format!("{output_path}.mp3");

    // Create the output file
    let mut output_file = File::create(&output_path)?;

    // Read the audio data and write it to the output file
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    output_file.write_all(&buffer)?;

    // Use ffmpeg to convert the audio file to mp3
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-vn")
        .arg("-ar")
        .arg("44100")
        .arg("-ac")
        .arg("2")
        .arg("-b:a")
        .arg("192k")
        .arg(&output_path)
        .output()?;

    if !output.status.success() {
        eprintln!("ffmpeg failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(output_path)
}