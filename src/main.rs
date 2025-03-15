use std::env;
use std::fs;
use std::path::Path;
use std::process;
use clipboard::{ClipboardContext, ClipboardProvider};
use walkdir::WalkDir;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: orzo <folder_path>");
        process::exit(1);
    }
    
    let folder_path = &args[1];
    
    if !Path::new(folder_path).is_dir() {
        eprintln!("Error: '{}' is not a valid directory", folder_path);
        process::exit(1);
    }
    
    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Failed to initialize clipboard");
    
    let mut all_content = String::new();
    
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        if path.is_file() {
            let path_str = path.to_string_lossy();
            
            if should_skip_file(path) {
                all_content.push_str(&format!("=========== {} (binary file - path only) =============\n", path_str));
                all_content.push_str("============================\n\n");
                continue;
            }
            
            match fs::read_to_string(path) {
                Ok(content) => {
                    all_content.push_str(&format!("=========== {} =============\n", path_str));
                    all_content.push_str(&content);
                    all_content.push_str("\n============================\n\n");
                }
                Err(_) => {
                    all_content.push_str(&format!("=========== {} (binary file - path only) =============\n", path_str));
                    all_content.push_str("============================\n\n");
                }
            }
        }
    }
    
    ctx.set_contents(all_content).expect("Failed to set clipboard contents");
    
    println!("Contents of files in '{}' copied to clipboard", folder_path);
}

fn should_skip_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        
        let skip_extensions = [
            "pdf", "jpg", "jpeg", "png", "gif", "bmp", "tiff", 
            "mp3", "mp4", "wav", "avi", "mov", "mkv", "webm",
            "exe", "dll", "so", "dylib", "bin", "dat", "db",
            "zip", "tar", "gz", "rar", "7z",
            "doc", "docx", "xls", "xlsx", "ppt", "pptx"
        ];
        
        if skip_extensions.contains(&ext.as_ref()) {
            return true;
        }
    }
    
    if let Ok(file) = fs::File::open(path) {
        let mut buffer = [0; 512];
        if let Ok(n) = file.take(512).read(&mut buffer) {
            for i in 0..n {
                if buffer[i] == 0 {
                    return true;
                }
            }
        }
    }
    
    false
}
