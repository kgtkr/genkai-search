use image::{
    imageops, DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageDecoder, ImageFormat,
    RgbImage, Rgba,
};
use std::fs::{read_dir, read_to_string, File};
use std::io::{stdin, BufRead, BufReader, BufWriter, Read, Write};
use std::process::{Child, Command, Stdio};

fn get_ss() -> DynamicImage {
    let ss = Command::new("adb")
        .arg("shell")
        .arg("screencap")
        .arg("-p")
        .output()
        .unwrap()
        .stdout;
    image::load_from_memory_with_format(&ss, ImageFormat::PNG).unwrap()
}

fn crop_char(image: &mut DynamicImage) -> DynamicImage {
    image.crop(490, 600, 100, 100)
}

fn crop_num(image: &mut DynamicImage) -> DynamicImage {
    image.crop(510, 720, 60, 80)
}

fn color_diff(c1: &Rgba<u8>, c2: &Rgba<u8>) -> f64 {
    let a = (c1.0[0] as f64 - c2.0[0] as f64).powi(2);
    let b = (c1.0[1] as f64 - c2.0[1] as f64).powi(2);
    let c = (c1.0[2] as f64 - c2.0[2] as f64).powi(2);
    let d = (c1.0[3] as f64 - c2.0[3] as f64).powi(2);
    (a + b + c + d).sqrt()
}

fn image_diff(image1: &DynamicImage, image2: &DynamicImage) -> f64 {
    let mut sum = 0.0;
    for x in 0..image1.width() {
        for y in 0..image1.height() {
            sum += color_diff(&image1.get_pixel(x, y), &image2.get_pixel(x, y));
        }
    }
    sum
}

fn read_images(dir: String) -> Vec<(String, DynamicImage)> {
    read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .map(|path| {
            (
                path.as_path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                image::open(path).unwrap(),
            )
        })
        .collect()
}

pub struct SSManager {
    chars: Vec<(char, DynamicImage)>,
    nums: Vec<(usize, DynamicImage)>,
}

impl SSManager {
    pub fn init() -> SSManager {
        SSManager {
            chars: read_images("learn-char".to_string())
                .into_iter()
                .map(|(x, image)| (x.chars().next().unwrap(), image))
                .collect(),
            nums: read_images("learn-num".to_string())
                .into_iter()
                .map(|(x, image)| (x.parse::<usize>().unwrap(), image))
                .collect(),
        }
    }

    pub fn cur(&self) -> (char, usize) {
        let mut ss = get_ss();
        let ss_num = crop_num(&mut ss);
        let ss_char = crop_char(&mut ss);
        (
            min_image(&ss_char, &self.chars),
            min_image(&ss_num, &self.nums),
        )
    }
}

fn min_image<T: Clone>(ss: &DynamicImage, images: &Vec<(T, DynamicImage)>) -> T {
    let mut ret = None;
    let mut min = None;
    for (key, diff) in images
        .iter()
        .map(|(key, image)| (key, image_diff(ss, image)))
    {
        if min.map(|min| min > diff).unwrap_or(true) {
            ret = Some(key);
            min = Some(diff);
        }
    }
    ret.unwrap().clone()
}

pub fn learn() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let mut image = get_ss();
    crop_char(&mut image)
        .save(format!("learn-char/{}.png", now))
        .unwrap();
    crop_num(&mut image)
        .save(format!("learn-num/{}.png", now))
        .unwrap();
}

pub fn hoge() -> String {
    let ss = Command::new("adb")
        .arg("shell")
        .arg("screencap")
        .arg("-p")
        .output()
        .unwrap()
        .stdout;
    BufWriter::new(File::create("tmp/screen.png").unwrap())
        .write_all(&ss)
        .unwrap();
    Command::new("tesseract")
        .arg("-l")
        .arg("jpn")
        .arg("tmp/screen.png")
        .arg("tmp/output")
        .output()
        .unwrap();
    read_to_string("tmp/output.txt").unwrap()
}
