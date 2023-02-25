mod math;

use std::{fs::File, io::Write, path::PathBuf};

pub fn save_buffer_to_ppm(
    path: PathBuf,
    buffer: &[u8],
    width: usize,
    height: usize,
) -> anyhow::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(b"P6\n")?;
    file.write(&format!("{} {}\n", width, height).into_bytes())?;
    file.write_all(b"255\n")?;
    file.write_all(buffer)?;
    Ok(())
}
