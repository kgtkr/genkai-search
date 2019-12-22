use super::keybord::{string_to_keys, Dire, InputButton};
use std::process::Command;

fn swipe(x1: i32, y1: i32, x2: i32, y2: i32) {
    Command::new("adb")
        .arg("shell")
        .arg("input")
        .arg("swipe")
        .arg(x1.to_string())
        .arg(y1.to_string())
        .arg(x2.to_string())
        .arg(y2.to_string())
        .output()
        .unwrap();
}

const swipe_width: i32 = 120;

fn key_to_point(key: &InputButton) -> (i32, i32) {
    match key {
        InputButton::A => (325, 1560),
        InputButton::K => (540, 1560),
        InputButton::S => (760, 1560),
        InputButton::T => (325, 1700),
        InputButton::N => (540, 1700),
        InputButton::H => (760, 1700),
        InputButton::M => (320, 1840),
        InputButton::Y => (540, 1840),
        InputButton::R => (760, 1840),
        InputButton::Point => (325, 2000),
        InputButton::W => (540, 2000),
        InputButton::Line => (760, 2000),
        InputButton::Enter => (110, 1560),
        InputButton::Send => (840, 1360),
    }
}

fn run_key((button, dire): &(InputButton, Dire)) {
    let (x1, y1) = key_to_point(button);
    let (dx, dy) = match dire {
        Dire::C => (0, 0),
        Dire::L => (-swipe_width, 0),
        Dire::U => (0, -swipe_width),
        Dire::R => (swipe_width, 0),
        Dire::D => (0, swipe_width),
    };
    swipe(x1, y1, x1 + dx, y1 + dy);
}

pub fn input_string(s: &String) {
    for key in string_to_keys(s).unwrap() {
        run_key(&key);
    }
}
