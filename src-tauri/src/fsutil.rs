use std::fs;
use std::io;
use std::path::Path;

/// Write a file atomically (temp file + rename) so a crash never leaves a
/// half-written profile/config file behind.
pub fn write_atomic(path: &Path, data: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension(format!(
        "{}.tmp",
        path.extension().and_then(|e| e.to_str()).unwrap_or("tmp")
    ));
    fs::write(&tmp, data)?;
    fs::rename(&tmp, path)?;
    Ok(())
}
