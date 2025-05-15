use std::str::FromStr as _;

use anyhow::{Context as _, Result};
use bluer::Address;
use image::{imageops::{dither, BiLevel, FilterType}, ImageReader};
use printer::Printer as _;

mod m02_constants;
mod printer;

pub(super) async fn print_image(mac: &str, image_addr: &str) -> Result<()> {
    let addr = Address::from_str(mac).context("Invalid address")?;

    // Загрузка изображения
    let image_reader = ImageReader::open(image_addr).context("Error loading image")?;
    println!("Reading image");

    let printer_fut = printer::M02ProPrinter::new(addr);

    let mut image = image_reader
        .decode()
        .context("Error decoding image")?;

    println!("Image decoded");
    if image.width() > image.height() {
        image = image.rotate90();
        println!("Rotated image on 90 degrees");
    }

    let mut printer = printer_fut.await.context("Error acquiring printer")?;
    image = image.resize(printer.get_print_width() as u32, (image.height() as f32 * m02_constants::IMG_WIDTH as f32 / image.width() as f32) as u32, FilterType::Lanczos3);
    println!("Image rescaled to {}x{}", image.width(), image.height());
    image = image.grayscale();
    println!("Image converted to grayscale");
    let mut image = image.as_mut_luma8().context("не смог преобразовать в luma8")?;
    println!("Image converted to luma8 buffered image");
    dither(&mut image, &BiLevel);
    println!("Image converted to B/W buffered image");

    printer.print_image(image).await?;
    Ok(())
}