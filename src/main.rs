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
    // Read CLI arguments
    let yaml_cli = load_yaml!("cli.yaml");
    let cli_args = App::from_yaml(yaml_cli).get_matches();

    let file_path_front = cli_args.value_of("front_image").unwrap();
    let file_path_back = cli_args.value_of("back_image").unwrap();
    let file_path_out = cli_args.value_of("output_image").unwrap();
    let overwrite_output_file = cli_args.is_present("overwrite_output_file");

    // check for possible errors when overwriting the output file
    let output_file = Path::new(file_path_out);
    if output_file.exists() && !overwrite_output_file {
        let msg = format!(
            "Path '{}' already exists! Use the -f option to overwrite it.",
            file_path_out
        );
        return Err(Box::new(IoError::new(IoErrorKind::AlreadyExists, msg)));
    } else if output_file.is_dir() && overwrite_output_file {
        let msg = format!(
            "Output path '{}' is a directory, cannot overwrite!",
            file_path_out
        );
        return Err(Box::new(IoError::new(IoErrorKind::AlreadyExists, msg)));
    }
    // Check for the output file extension and give the user a warning if it's not "png"
    // this single line is so ugly that it makes me never wanna touch rust again
    let file_out_ext = output_file.extension();
    if file_out_ext != None {
        let ext_str = file_out_ext.unwrap().to_str().unwrap().to_lowercase();
        if ext_str != "png" {
            eprintln!("Warning: output file's extension is not 'png'. The saved file will be a PNG, regardless of the output extension!");
        }
    }

    let in_front = load_and_resize(file_path_front, FRONT_SIZE)?;
    let in_back = load_and_resize(file_path_back, BACK_SIZE)?;
    let mut canvas: RgbaImage = ImageBuffer::new(64, 64);

    copy_back_regions(&in_back, &mut canvas);
    copy_front_regions(&in_front, &mut canvas);

    canvas.save_with_format(output_file, ImageFormat::Png)?;

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
    // Head (top, right, left, front)
    copy_region(src, dest, (0, 0), (10, 0), (24, 16));
    // Head under
    copy_region_180deg(src, dest, (16, 0), (18, 16), (8, 8));
    // Body front, left, right
    copy_region(src, dest, (16, 20), (14, 16), (16, 12));
    // Body top
    copy_region(src, dest, (20, 16), (18, 12), (8, 4));
    // Body under
    copy_region(src, dest, (28, 16), (18, 28), (8, 4));
    // right arm outter, front, inner
    copy_region(src, dest, (40, 20), (10, 16), (12, 12));
    // right arm top
    copy_region(src, dest, (44, 16), (14, 12), (4, 4));
    // right arm bottom
    copy_region_180deg(src, dest, (48, 16), (14, 28), (4, 4));
    // left arm inner, front, outter
    copy_region(src, dest, (32, 52), (22, 16), (12, 12));
    // left arm top
    copy_region(src, dest, (36, 48), (26, 12), (4, 4));
    // left arm bottom
    copy_region_180deg(src, dest, (40, 48), (26, 28), (4, 4));
    // right leg outter, front, inner
    copy_region(src, dest, (0, 20), (14, 28), (12, 12));
    // right leg bottom
    copy_region_180deg(src, dest, (8, 16), (18, 40), (4, 4));
    // right leg top
    copy_region(src, dest, (4, 16), (18, 24), (4, 4));
    // left leg inner, front, outter
    copy_region(src, dest, (16, 52), (18, 28), (12, 12));
    // left leg bottom
    copy_region_180deg(src, dest, (24, 48), (22, 40), (4, 4));
    // left leg top
    copy_region(src, dest, (20, 48), (22, 24), (4, 4));
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
