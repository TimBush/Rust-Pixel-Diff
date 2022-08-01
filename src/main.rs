use image::{io::Reader as ImageReader, GenericImageView, RgbImage, Rgb};
use png_diff::cli::parse_args;

const Y_RVAL: f64 = 0.29889531;
const Y_GVAL: f64 = 0.58662247;
const Y_BVAL: f64 = 0.11448223;

const I_RVAL: f64 = 0.59597799;
const I_GVAL: f64 = 0.27417610;
const I_BVAL: f64 = 0.32180189;

const Q_RVAL: f64 = 0.21147017;
const Q_GVAL: f64 = 0.52261711;
const Q_BVAL: f64 = 0.31114694;


fn main() {
    let args = parse_args();

    let img = ImageReader::open(args.img1_name).unwrap().decode().unwrap();
    let img2 = ImageReader::open(args.img2_name).unwrap().decode().unwrap();
    let mut rgb_img = img.pixels();
    let mut rgb_img2 = img2.pixels();

    let max_delta = 35215.0 * args.threshold * args.threshold;
    let mut diff: Vec<(u32, u32)> = Vec::new();

    let num_of_pixels = img.height() * img.width();
    for i in 0..num_of_pixels {
        let (x1, y1, rgb1) = rgb_img.next().unwrap();
        let (x2, y2, rgb2) = rgb_img2.next().unwrap();

        // Make sure x & y coords match
        if x1 == x2 && y1 == y2 {
            let delta_E = calculate_delta_E(rgb1.0, rgb2.0);
            if delta_E.abs() > max_delta {
                diff.push((x1, y1));
            }
        }

    }
    generate_new_img(diff, img.width(), img.height(), args.output_dir);
}

fn generate_new_img(coords: Vec<(u32, u32)>, width: u32, height: u32, output_dir: String) {
    let mut img = RgbImage::new(width, height);
    for i in 0..coords.len() {
        img.put_pixel(coords[i].0, coords[i].1, Rgb([255, 0, 0]));
    }
    img.save(format!("{}/output.png", output_dir)).unwrap();
}

// Calculates the Delta E between color_1 & color_2
// Lets us then determine what's an acceptable threshold for the difference in pixel colors between the 2 images
fn calculate_delta_E(color_1: [u8; 4], color_2: [u8; 4]) -> f64 {
    let mut r1 = color_1[0] as f64;
    let mut g1 = color_1[1] as f64;
    let mut b1 = color_1[2] as f64;
    let mut a1 = color_1[3] as f64;

    let mut r2 = color_2[0] as f64;
    let mut g2 = color_2[1] as f64;
    let mut b2 = color_2[2] as f64;
    let mut a2 = color_2[3] as f64;

    if a1 == a2 && r1 == r2 && g1 == g2 && b1 == b2 { 
       return 0.0
    }

    if a1 < 255.0 {
        a1 /= 255.0;
        r1 = blend(r1, a1);
        g1 = blend(g1, a1);
        b1 = blend(b1, a1);
    }

    if a2 < 255.0 {
        a2 /= 255.0;
        r2 = blend(r2, a2);
        g2 = blend(g2, a2);
        b2 = blend(b2, a2);
    }

    let y1 = rgb2y(r1, g1, b1);
    let y2 = rgb2y(r2, g2, b2);
    let y = y1 - y2;

    let i = rgb2i(r1, g1, b1) - rgb2i(r1, g2, b2);
    let q = rgb2q(r1, g1, b1) - rgb2q(r2, g2, b2);

    let delta = 0.5053 * y * y + 0.299 * i * i + 0.1957 * q * q;

    if y1 > y2 {
        -delta
    } else {
        delta
    } 
}


fn rgb2y(r: f64, g: f64, b: f64) -> f64 {
    r * Y_RVAL + g * Y_GVAL + b * Y_BVAL
}
fn rgb2i(r: f64, g: f64, b: f64) -> f64 {
    r * I_RVAL + g * I_GVAL + b * I_BVAL
}
fn rgb2q(r: f64, g: f64, b: f64) -> f64 {
    r * Q_RVAL + g * Q_GVAL + b * Q_BVAL
}

// blend semi-transparent color with white
fn blend(c: f64, a: f64) -> f64 {
    255.0 + (c - 255.0) * a
}