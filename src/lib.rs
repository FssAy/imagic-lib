mod seed;

pub use image::error;
use std::ops::Range;
use image::{DynamicImage, GenericImageView, guess_format, ImageError, ImageFormat, Rgba};
use error::{ImageFormatHint, UnsupportedError};
use rand::Rng;
use rand_pcg::Pcg64;
pub use seed::Seed;
use crate::Imagic::{Decrypt, Encrypt};


type Matrix = Vec<Vec<[u32; 2]>>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Imagic {
    Encrypt,
    Decrypt,
}

impl Imagic {
    fn gen_pos_matrix<R: Rng>(&self, rng: &mut R, dim: (u32, u32)) -> Matrix {
        let mut matrix = Matrix::new();

        for _y in 0..dim.1 {
            let mut row = Vec::new();
            for _x in 0..dim.0 {
                row.push([
                    rng.gen_range(0..dim.0),
                    rng.gen_range(0..dim.1),
                ]);
            }
            if self == &Decrypt { row.reverse(); }
            matrix.push(row);
        }

        if self == &Decrypt { matrix.reverse(); }
        matrix
    }

    fn iter_over(&self, range: Range<u32>) -> Box<dyn Iterator<Item = u32>> {
        if self == &Encrypt {
            Box::new(range)
        } else {
            Box::new(range.rev())
        }
    }

    unsafe fn swap_pos_by_matrix(&self, img: &mut DynamicImage, matrix: &Vec<Vec<[u32; 2]>>, pos: (u32, u32), pixel: Rgba<u8>) {
        use image::GenericImage;

        let pos_x = match self {
            Encrypt => pos.0,
            Decrypt => img.dimensions().0-1 - pos.0,
        };

        let pos_y = match self {
            Encrypt => pos.1,
            Decrypt => img.dimensions().1-1 - pos.1,
        };

        let new_pos = matrix.get_unchecked(pos_y as usize).get_unchecked(pos_x as usize);
        let pixel_target = img.unsafe_get_pixel(new_pos[0], new_pos[1]);

        // println!("moved from {:?} into {:?}", pos, new_pos);

        img.put_pixel(new_pos[0], new_pos[1], pixel);
        img.put_pixel(pos.0, pos.1, pixel_target);
    }

    pub fn from_buffer<B: AsRef<[u8]>>(&self, buf: B, seed: Seed, quality: u8) -> Result<Vec<u8>, ImageError> {
        let mut rng: Pcg64 = seed.into();

        let format = guess_format(buf.as_ref())?;

        match format {
            ImageFormat::Png => {}
            ImageFormat::Jpeg => {}
            ImageFormat::Bmp => {}
            _ => {
                return Err(
                    ImageError::Unsupported(
                        UnsupportedError::from(ImageFormatHint::Exact(format))
                    )
                );
            }
        }

        let mut img = image::load_from_memory_with_format(buf.as_ref(), format)?;

        drop(buf);

        let dim = img.dimensions();
        let matrix = self.gen_pos_matrix(&mut rng, dim);

        for y in self.iter_over(0..dim.1) {
            for x in self.iter_over(0..dim.0) {
                unsafe {
                    let pixel = img.unsafe_get_pixel(x, y);
                    self.swap_pos_by_matrix(&mut img, &matrix, (x, y), pixel);
                }
            }
        }

        let mut buf = Vec::new();

        match format {
            ImageFormat::Png => {
                use image::codecs::png::{CompressionType, FilterType, PngEncoder};
                let compression = match quality {
                    0 => CompressionType::Default,
                    1 => CompressionType::Fast,
                    2 => CompressionType::Best,
                    3 => CompressionType::Rle,
                    _ => CompressionType::Huffman,
                };
                PngEncoder::new_with_quality(&mut buf, compression, FilterType::Avg)
                    .encode(img.as_bytes(), img.width(), img.height(), img.color())?;
            }
            ImageFormat::Jpeg => {
                use image::codecs::jpeg::JpegEncoder;
                JpegEncoder::new_with_quality(&mut buf, quality).encode_image(&img)?;
            }
            ImageFormat::Bmp => {
                use image::codecs::bmp::BmpEncoder;
                BmpEncoder::new(&mut buf).encode(img.as_bytes(), img.width(), img.height(), img.color())?;
            }
            _ => unreachable!()
        }

        Ok(buf)
    }
}
