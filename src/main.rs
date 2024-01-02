use image::{imageops::FilterType, DynamicImage, GenericImageView, Rgba};
use lazy_static::lazy_static;
use std::env;

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

    fn distance(left: &Rgba<u8>, right: &Rgba<u8>) -> f64 {
        let red_difference = f64::from(left[0]) - f64::from(right[0]);
        let green_difference = f64::from(left[1]) - f64::from(right[1]);
        let blue_difference = f64::from(left[2]) - f64::from(right[2]);
        (red_difference.powi(2) + green_difference.powi(2) + blue_difference.powi(2)).sqrt()
    }

    fn closest(other: &Rgba<u8>) -> &String {
        let mut closest_distance = CliPixel::distance(other, &COLORS.get(0).unwrap().rgba);
        let mut closest_index = 0;
        for i in 1..COLORS.len() {
            let current_element = COLORS.get(i).unwrap();
            let current_distance = CliPixel::distance(other, &current_element.rgba);
            if current_distance < closest_distance {
                closest_distance = current_distance;
                closest_index = i;
            }
        }
        &COLORS.get(closest_index).unwrap().code
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
                "{} {}",
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
        CliPixel::new(String::from("\x1b[1;40m"), image::Rgba([0, 0, 0, 1])),
        CliPixel::new(String::from("\x1b[1;41m"), image::Rgba([255, 0, 0, 1])),
        CliPixel::new(String::from("\x1b[1;42m"), image::Rgba([0, 255, 0, 1])),
        CliPixel::new(String::from("\x1b[1;43m"), image::Rgba([255, 255, 0, 1])),
        CliPixel::new(String::from("\x1b[1;44m"), image::Rgba([0, 0, 255, 1])),
        CliPixel::new(String::from("\x1b[1;45m"), image::Rgba([255, 0, 255, 1])),
        CliPixel::new(String::from("\x1b[1;46m"), image::Rgba([0, 255, 255, 1])),
        CliPixel::new(String::from("\x1b[1;47m"), image::Rgba([255, 255, 255, 1]))
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
