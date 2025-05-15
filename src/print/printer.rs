use std::io::Read as _;

use anyhow::{Context as _, Result};

use bluer::{rfcomm::{Socket, SocketAddr, Stream}, Address};
use image::{ImageBuffer, Luma};
use tokio::io::AsyncWriteExt;

use super::m02_constants::*;

pub(super) trait Printer {
    async fn print_image(&mut self, image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Result<()>;
    fn get_print_width(&self) -> usize {
        IMG_WIDTH
    }
}

pub(super) struct M02ProPrinter {
    stream: Stream,
}

impl M02ProPrinter {
    pub(super) async fn new(addr: Address) -> Result<Self> {
        // Создаем RFCOMM-клиент
        let connector = Socket::new().context("Failed to get socket")?;
        let stream = connector.connect(SocketAddr::new(addr, 1)).await.context("Failed to connect to the printer")?;
        println!("Connected to the printer");
        Ok(Self { stream })
    }

    async fn send_header(&mut self) -> Result<()> {
        self.stream.write_all(_INIT).await?;
        self.stream.write_all(_ALIGN_CENTER).await?;
        self.stream.write_all(PROP_MAGIC_HEADER).await?; //look lighter if not used (colour density?)
        Ok(())
    }
    
    async fn send_graphic_mode(&mut self, height: u16) -> Result<()> {
        let mut buf = [0u8; 8];
        buf[0] = _GS_MODE[0];
        buf[1] = _GS_MODE[1];
        buf[2] = _GS_MODE[2];
        buf[3] = _GS_MODE[3];
        buf[4] = _WIDTH[0];
        buf[5] = _WIDTH[1];
        buf[6] = (height % 256) as u8;
        buf[7] = (height / 256) as u8;
        self.stream.write_all(&buf).await?;
        Ok(())
    }
    
    async fn send_image(&mut self, image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Result<()> {
        let mut bit = 0;
        let mut buffer = [0; LINE_BUFFER_LENGTH];
        let mut pos = 0;
        let mut processed = 0;
        for pix in image.bytes() {
            processed += 1;
            print!("\r{processed} / {}", image.len());
            if pix.unwrap() == 0 {
                buffer[pos] |= 1 << (7 - bit);
            }
            if bit < 7 {
                bit += 1;
            } else {
                bit = 0;
                if pos == buffer.len() - 1 {
                    self.stream.write_all(&buffer).await?;
                    pos = 0;
                    buffer = [0; LINE_BUFFER_LENGTH];
                } else {
                    pos += 1;
                }
            }
        }
        Ok(())
    }
    
    async fn send_footer(&mut self) -> Result<()> {
        self.stream.write_all(_PAPER_FEED_4).await?;
    //    stream.write_all(PROP_MAGIC_FOOTER).await?;
        Ok(())
    }
}

impl Printer for M02ProPrinter {
    async fn print_image(&mut self, image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Result<()> {
        self.send_header().await?;
        self.send_graphic_mode(image.height() as u16).await?;
        self.send_image(image).await?;
        self.send_footer().await?;
        Ok(())
    }
}
