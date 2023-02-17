use std::{fs::File, io::Write};

// TODO: FIX THIS BS
pub fn save_buffer_to_ppm(
    path: &str,
    buffer: &[u8],
    width: usize,
    height: usize,
) -> anyhow::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(b"P6\n")?;
    file.write_all(b"255\n")?;
    file.write(&format!("{} {}\n", height, width).into_bytes())?;
    file.write_all(buffer)?;
    Ok(())
}
