use super::keybord::{string_to_keys, Dire, InputButton};
use std::io::Write;
use std::process::{Child, Command, Stdio};

pub struct KeyboardManager(Child);

impl KeyboardManager {
    pub fn start() -> KeyboardManager {
        KeyboardManager(
            Command::new("adb")
                .arg("shell")
                .stdout(Stdio::null())
                .stdin(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .unwrap(),
        )
    }

    fn swipe(&mut self, (x1, y1, x2, y2): &(i32, i32, i32, i32)) {
        let stdin = self.0.stdin.as_mut().unwrap();
        stdin
            .write(format!("input swipe {} {} {} {}\n", x1, y1, x2, y2).as_bytes())
            .unwrap();
        stdin.flush().unwrap();
    }

    pub fn input_string(&mut self, s: &String) {
        for key in string_to_keys(s).unwrap() {
            self.swipe(&key_to_swipe(&key));
        }
    }
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
