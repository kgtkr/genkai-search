use image::{
    imageops, DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageDecoder, ImageFormat,
    RgbImage,
};
use std::fs::{read_to_string, File};
use std::io::{stdin, BufRead, BufReader, BufWriter, Read, Write};
use std::process::{Child, Command, Stdio};

fn save_ss() -> DynamicImage {
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
    image::load_from_memory_with_format(&ss, ImageFormat::PNG).unwrap()
}

fn crop_char(image: &mut DynamicImage) -> DynamicImage {
    image.crop(490, 600, 100, 100)
}

fn crop_num(image: &mut DynamicImage) -> DynamicImage {
    image.crop(510, 720, 60, 80)
}

pub fn foo() {
    let mut image = save_ss();
    image.crop(490, 600, 100, 100).save("tmp/char.png").unwrap();
}

pub fn learn() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let mut image = save_ss();
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
