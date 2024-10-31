use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use walkdir::WalkDir;

#[derive(Debug)]
struct ModuleProcessor {
    processed_files: HashSet<PathBuf>,
    result: Vec<String>,
}

impl ModuleProcessor {
    fn new() -> Self {
        Self {
            processed_files: HashSet::new(),
            result: Vec::new(),
        }
    }

    fn process_rust_file(&self, file_path: &Path) -> io::Result<String> {
        let content = fs::read_to_string(file_path)?;
        
        // Add a file separator comment for better readability
        let separator = format!(
            "// File: {}\n",
            file_path.display()
        );
        
        Ok(format!("{}{}\n\n", separator, content.trim()))
    }

    fn flatten_directory(&mut self, directory_path: &Path) -> io::Result<String> {
        // Walk through all files in the directory
        for entry in WalkDir::new(directory_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        {
            let file_path = entry.path();
            
            if !self.processed_files.contains(file_path) {
                self.processed_files.insert(file_path.to_path_buf());
                let content = self.process_rust_file(file_path)?;
                if !content.trim().is_empty() {
                    self.result.push(content);
                }
            }
        }

        Ok(self.result.join(""))
    }
}

fn concatenate_directory(
    directory_path: &Path,
    output_file: &Path,
) -> io::Result<()> {
    let mut processor = ModuleProcessor::new();
    let concatenated_content = processor.flatten_directory(directory_path)?;

    // Create the output directory if it doesn't exist
    if let Some(parent) = output_file.parent() {
        fs::create_dir_all(parent)?;
    }

    // Add header comment
    let header = format!(
        "// Concatenated Rust files from directory: {}\n\
         // Generated automatically on {}\n\
         // This file contains all Rust code from the specified directory\n\n",
        directory_path.display(),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    // Write to output file
    let mut file = fs::File::create(output_file)?;
    file.write_all(header.as_bytes())?;
    file.write_all(concatenated_content.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_to_flatten> [output_file]", args[0]);
        eprintln!("Example: {} src/models combined_models.rs", args[0]);
        std::process::exit(1);
    }

    let directory_path = Path::new(&args[1]);
    let output_path = if args.len() >= 3 {
        PathBuf::from(&args[2])
    } else {
        PathBuf::from("flattened_code.rs")
    };

    if !directory_path.exists() {
        eprintln!("Error: Directory '{}' does not exist", directory_path.display());
        std::process::exit(1);
    }

    match concatenate_directory(directory_path, &output_path) {
        Ok(()) => println!(
            "Successfully flattened Rust files from '{}' into: '{}'",
            directory_path.display(),
            output_path.display()
        ),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}