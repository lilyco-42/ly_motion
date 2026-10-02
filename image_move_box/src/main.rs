use image::ImageReader;
use std::{fs, io::Cursor};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = fs::read("./demo.png").unwrap();
    let img = ImageReader::open("myimage.png")?.decode()?;
    let img2 = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;

    img.save("empty.jpg")?;

    let mut bytes: Vec<u8> = Vec::new();
    img2.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)?;
    Ok(())
}
