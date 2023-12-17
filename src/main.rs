use image::{imageops::FilterType, DynamicImage, GenericImageView, Rgba};
use lazy_static::lazy_static;
use ordered_float::OrderedFloat;
use std::{collections::BTreeMap, env};

const MAX_BOUNDARY: u32 = 100;
const MIN_BOUNDARY: u32 = 10;

struct CliPixel {
    code: String,
    rgba: Rgba<u8>,
}

impl CliPixel {
    fn new(code: String, rgba: Rgba<u8>) -> Self {
        Self { code, rgba }
    }

    fn distance(left: &Rgba<u8>, right: &Rgba<u8>) -> OrderedFloat<f64> {
        let red_difference = f64::from(left[0]) - f64::from(right[0]);
        let green_difference = f64::from(left[1]) - f64::from(right[1]);
        let blue_difference = f64::from(left[2]) - f64::from(right[2]);
        OrderedFloat(
            (red_difference.powi(2) + green_difference.powi(2) + blue_difference.powi(2)).sqrt(),
        )
    }

    fn closest(other: &Rgba<u8>) -> &String {
        let mut tree = BTreeMap::new();
        for color in COLORS.iter() {
            let distance = CliPixel::distance(other, &color.rgba);
            tree.insert(distance, &color.code);
        }
        *tree.values().next().unwrap()
    }
}

fn open_scale_image(image: &DynamicImage) -> DynamicImage {
    let (new_width, new_height) = scale(image.dimensions());
    image.resize(new_width, new_height, FilterType::Lanczos3)
}

fn scale(original_boundary: (u32, u32)) -> (u32, u32) {
    (
        std::cmp::max(
            std::cmp::min(original_boundary.0, MAX_BOUNDARY),
            MIN_BOUNDARY,
        ),
        std::cmp::max(
            std::cmp::min(original_boundary.1, MAX_BOUNDARY),
            MIN_BOUNDARY,
        ),
    )
}

fn print_image(image: &DynamicImage) {
    let (width, height) = image.dimensions();
    for y in 0..height {
        for x in 0..width {
            print!(
                "{}#{}",
                CliPixel::closest(&image.get_pixel(x, y)),
                *RESET_ANSI
            );
        }
        println!();
    }
    println!("{}, {}", width, height);
}

lazy_static! {
    static ref COLORS: Vec<CliPixel> = vec![
        CliPixel::new(String::from("\x1b[1;30m"), image::Rgba([0, 0, 0, 1])),
        CliPixel::new(String::from("\x1b[1;31m"), image::Rgba([255, 0, 0, 1])),
        CliPixel::new(String::from("\x1b[1;32m"), image::Rgba([0, 255, 0, 1])),
        CliPixel::new(String::from("\x1b[1;33m"), image::Rgba([255, 255, 0, 1])),
        CliPixel::new(String::from("\x1b[1;34m"), image::Rgba([0, 0, 255, 1])),
        CliPixel::new(String::from("\x1b[1;35m"), image::Rgba([255, 0, 255, 1])),
        CliPixel::new(String::from("\x1b[1;36m"), image::Rgba([0, 255, 255, 1])),
        CliPixel::new(
            String::from("\x1b[1;37m"),
            image::Rgba([255, 255, 255, 255])
        )
    ];
    static ref RESET_ANSI: String = String::from("\x1b[0m");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Image file missing!");
        std::process::exit(1);
    }

    let image = open_scale_image(&image::open(&args[0]).unwrap());
    print_image(&image);
}
