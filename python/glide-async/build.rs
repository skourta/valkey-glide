// Copyright Valkey GLIDE Project Contributors - SPDX Identifier: Apache-2.0

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let source_dir = Path::new("../glide-shared/glide_shared");    
    let dest_dir = Path::new("./python/glide_shared");
    println!("cargo:rerun-if-changed={}", source_dir.display());

    if source_dir.exists() {
        // Run the recursive copy
        if let Err(e) = copy_dir_all(source_dir, dest_dir) {
            // Panic ensures the build fails if the copy fails (so you don't ship empty wheels)
            panic!("Failed to copy glide-shared: {}", e);
        }
    } else {
        println!("cargo:warning=Parent directory '../glide-shared' not found.");
    }
}

// --- Helper: Recursive Copy Function ---
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), &dest_path)?;
        }
    }
    Ok(())
}