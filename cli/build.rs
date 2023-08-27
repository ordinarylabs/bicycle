use std::fs;
use std::io;

fn copy_dir(src_dir: String, dest_dir: String) -> io::Result<()> {
    fs::create_dir_all(dest_dir.clone())?;

    let entries = fs::read_dir(src_dir)?;

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();
        let file_name = entry_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let dest_path = format!("{}/{}", dest_dir, file_name);

        if entry_path.is_dir() {
            copy_dir(entry_path.to_string_lossy().to_string(), dest_path.clone())?;
        } else if entry_path.is_file() {
            fs::copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    copy_dir("../core".to_string(), "./tmp/core".to_string())?;
    copy_dir("../server".to_string(), "./tmp/server".to_string())?;
    fs::copy("../Cargo.toml", "./tmp/Cargo.toml")?;

    Ok(())
}
