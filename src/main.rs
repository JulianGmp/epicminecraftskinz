extern crate image;

use clap::{load_yaml, App};
use image::imageops::FilterType;
use image::io::Reader as ImgReader;
use image::{ImageBuffer, ImageFormat, RgbaImage};
use std::error::Error;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::path::Path;

const FRONT_SIZE: (u32, u32) = (44, 44);
const BACK_SIZE: (u32, u32) = (32, 32);

fn main() -> Result<(), Box<dyn Error>> {
    let yaml_cli = load_yaml!("cli.yaml");
    let cli_args = App::from_yaml(yaml_cli).get_matches();

    let file_front = cli_args.value_of("front_image").unwrap();
    let file_back = cli_args.value_of("back_image").unwrap();
    let file_out = cli_args.value_of("output_image").unwrap();

    if Path::new(file_out).exists() {
        let msg = format!("File '{}' already exists!", file_out);
        return Err(Box::new(IoError::new(IoErrorKind::AlreadyExists, msg)));
    }
    // this single line is so ugly that it makes me never wanna touch rust again
    let file_out_ext = Path::new(file_out).extension().unwrap().to_str().unwrap().to_lowercase();
    if file_out_ext != "png" {
        eprintln!("Warning: output file's extension is not 'png'. The saved file will be a png, regardless of the output extension!");
    }

    let in_front = load_and_resize(file_front, FRONT_SIZE)?;
    let in_back = load_and_resize(file_back, BACK_SIZE)?;
    let mut canvas: RgbaImage = ImageBuffer::new(64, 64);

    copy_back_regions(&in_back, &mut canvas);
    copy_front_regions(&in_front, &mut canvas);

    canvas.save_with_format(file_out, ImageFormat::Png)?;

    return Ok(());
}

fn load_and_resize(path: &str, size: (u32, u32)) -> Result<RgbaImage, Box<dyn Error>> {
    let img = ImgReader::open(path)?.decode()?;
    return Ok(img
        .resize_exact(size.0, size.1, FilterType::Nearest)
        .into_rgba());
}

fn copy_back_regions(src: &RgbaImage, dest: &mut RgbaImage) {
    // head
    copy_region(src, dest, (24, 8), (12, 0), (8, 8));
    // body
    copy_region(src, dest, (32, 20), (12, 8), (8, 12));
    // right arm
    copy_region(src, dest, (44, 52), (8, 8), (4, 12));
    // left arm
    copy_region(src, dest, (52, 20), (20, 8), (4, 12));
    // right leg
    copy_region(src, dest, (28, 52), (12, 20), (4, 12));
    // left leg
    copy_region(src, dest, (12, 20), (16, 20), (4, 12));
}

fn copy_front_regions(src: &RgbaImage, dest: &mut RgbaImage) {
    // Head regions (TRFL + U)
    copy_region(src, dest, (0, 0), (10, 0), (24, 16));
    copy_region_180deg(src, dest, (16, 0), (18, 16), (8, 8));
    // Body front
    copy_region(src, dest, (20, 20), (18, 16), (8, 12));
    // right arm
    copy_region(src, dest, (40, 20), (10, 16), (8, 12));
    copy_region(src, dest, (44, 16), (14, 12), (4, 4));
    copy_region_180deg(src, dest, (48, 16), (14, 28), (4, 4));
    // left arm
    copy_region(src, dest, (36, 52), (26, 16), (8, 12));
    copy_region(src, dest, (36, 48), (26, 12), (4, 4));
    copy_region_180deg(src, dest, (40, 48), (26, 28), (4, 4));
    // right leg
    copy_region(src, dest, (0, 20), (14, 28), (8, 12));
    copy_region_180deg(src, dest, (8, 16), (18, 40), (4, 4));
    // left leg
    copy_region(src, dest, (20, 52), (22, 28), (8, 12));
    copy_region_180deg(src, dest, (24, 48), (22, 40), (4, 4));
}

fn copy_region(
    src: &RgbaImage,
    dest: &mut RgbaImage,
    dest_pos: (u32, u32),
    src_pos: (u32, u32),
    size: (u32, u32),
) {
    for d_x in 0..size.0 {
        for d_y in 0..size.1 {
            dest.put_pixel(
                dest_pos.0 + d_x,
                dest_pos.1 + d_y,
                *src.get_pixel(src_pos.0 + d_x, src_pos.1 + d_y),
            )
        }
    }
}

fn copy_region_180deg(
    src: &RgbaImage,
    dest: &mut RgbaImage,
    dest_pos: (u32, u32),
    src_pos: (u32, u32),
    size: (u32, u32),
) {
    for d_x in 0..size.0 {
        for d_y in 0..size.1 {
            dest.put_pixel(
                dest_pos.0 + d_x,
                dest_pos.1 + d_y,
                *src.get_pixel(src_pos.0 + size.0 - 1 - d_x, src_pos.1 + size.1 - 1 - d_y),
            )
        }
    }
}
