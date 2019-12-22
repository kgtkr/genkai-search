use super::keybord::{string_to_keys, Dire, InputButton};
use std::process::Command;

fn swipe(list: &Vec<(i32, i32, i32, i32)>) {
    Command::new("adb")
        .arg("shell")
        .arg(
            list.iter()
                .map(|(x1, y1, x2, y2)| format!("input swipe {} {} {} {}", x1, y1, x2, y2))
                .collect::<Vec<_>>()
                .join("&&"),
        )
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

fn key_to_swipe((button, dire): &(InputButton, Dire)) -> (i32, i32, i32, i32) {
    let (x1, y1) = key_to_point(button);
    let (dx, dy) = match dire {
        Dire::C => (0, 0),
        Dire::L => (-swipe_width, 0),
        Dire::U => (0, -swipe_width),
        Dire::R => (swipe_width, 0),
        Dire::D => (0, swipe_width),
    };
    (x1, y1, x1 + dx, y1 + dy)
}

pub fn input_string(s: &String) {
    swipe(
        &string_to_keys(s)
            .unwrap()
            .into_iter()
            .map(|key| key_to_swipe(&key))
            .collect::<Vec<_>>(),
    );
}
