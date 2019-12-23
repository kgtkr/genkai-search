#[derive(Clone, PartialEq, Debug)]
pub enum InputButton {
    A,
    K,
    S,
    T,
    N,
    H,
    M,
    Y,
    R,
    Point,
    W,
    Line,
    Enter,
    Send,
    Delete,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Dire {
    C,
    L,
    U,
    R,
    D,
}

fn char_to_keys(c: char) -> Option<Vec<(InputButton, Dire)>> {
    Some(match c {
        'ァ' => vec![(InputButton::A, Dire::C), (InputButton::Point, Dire::C)],
        'ア' => vec![(InputButton::A, Dire::C)],
        'ィ' => vec![(InputButton::A, Dire::L), (InputButton::Point, Dire::C)],
        'イ' => vec![(InputButton::A, Dire::L)],
        'ゥ' => vec![(InputButton::A, Dire::U), (InputButton::Point, Dire::C)],
        'ウ' => vec![(InputButton::A, Dire::U)],
        'ヴ' => vec![(InputButton::A, Dire::U), (InputButton::Point, Dire::C)],
        'ェ' => vec![(InputButton::A, Dire::R), (InputButton::Point, Dire::C)],
        'エ' => vec![(InputButton::A, Dire::R)],
        'ォ' => vec![(InputButton::A, Dire::D), (InputButton::Point, Dire::C)],
        'オ' => vec![(InputButton::A, Dire::D)],
        'カ' => vec![(InputButton::K, Dire::C)],
        'ガ' => vec![(InputButton::K, Dire::C), (InputButton::Point, Dire::C)],
        'キ' => vec![(InputButton::K, Dire::L)],
        'ギ' => vec![(InputButton::K, Dire::L), (InputButton::Point, Dire::C)],
        'ク' => vec![(InputButton::K, Dire::U)],
        'グ' => vec![(InputButton::K, Dire::U), (InputButton::Point, Dire::C)],
        'ケ' => vec![(InputButton::K, Dire::R)],
        'ゲ' => vec![(InputButton::K, Dire::R), (InputButton::Point, Dire::C)],
        'コ' => vec![(InputButton::K, Dire::D)],
        'ゴ' => vec![(InputButton::K, Dire::D), (InputButton::Point, Dire::C)],
        'サ' => vec![(InputButton::S, Dire::C)],
        'ザ' => vec![(InputButton::S, Dire::C), (InputButton::Point, Dire::C)],
        'シ' => vec![(InputButton::S, Dire::L)],
        'ジ' => vec![(InputButton::S, Dire::L), (InputButton::Point, Dire::C)],
        'ス' => vec![(InputButton::S, Dire::U)],
        'ズ' => vec![(InputButton::S, Dire::U), (InputButton::Point, Dire::C)],
        'セ' => vec![(InputButton::S, Dire::R)],
        'ゼ' => vec![(InputButton::S, Dire::R), (InputButton::Point, Dire::C)],
        'ソ' => vec![(InputButton::S, Dire::D)],
        'ゾ' => vec![(InputButton::S, Dire::D), (InputButton::Point, Dire::C)],
        'タ' => vec![(InputButton::T, Dire::C)],
        'ダ' => vec![(InputButton::T, Dire::C), (InputButton::Point, Dire::C)],
        'チ' => vec![(InputButton::T, Dire::L)],
        'ヂ' => vec![(InputButton::T, Dire::L), (InputButton::Point, Dire::C)],
        'ッ' => vec![(InputButton::T, Dire::U), (InputButton::Point, Dire::C)],
        'ツ' => vec![(InputButton::T, Dire::U)],
        'ヅ' => vec![
            (InputButton::T, Dire::U),
            (InputButton::Point, Dire::C),
            (InputButton::Point, Dire::C),
        ],
        'テ' => vec![(InputButton::T, Dire::R)],
        'デ' => vec![(InputButton::T, Dire::R), (InputButton::Point, Dire::C)],
        'ト' => vec![(InputButton::T, Dire::D)],
        'ド' => vec![(InputButton::T, Dire::D), (InputButton::Point, Dire::C)],
        'ナ' => vec![(InputButton::N, Dire::C)],
        'ニ' => vec![(InputButton::N, Dire::L)],
        'ヌ' => vec![(InputButton::N, Dire::U)],
        'ネ' => vec![(InputButton::N, Dire::R)],
        'ノ' => vec![(InputButton::N, Dire::D)],
        'ハ' => vec![(InputButton::H, Dire::C)],
        'バ' => vec![(InputButton::H, Dire::C), (InputButton::Point, Dire::C)],
        'パ' => vec![
            (InputButton::H, Dire::C),
            (InputButton::Point, Dire::C),
            (InputButton::Point, Dire::C),
        ],
        'ヒ' => vec![(InputButton::H, Dire::L)],
        'ビ' => vec![(InputButton::H, Dire::L), (InputButton::Point, Dire::C)],
        'ピ' => vec![
            (InputButton::H, Dire::L),
            (InputButton::Point, Dire::C),
            (InputButton::Point, Dire::C),
        ],
        'フ' => vec![(InputButton::H, Dire::U)],
        'ブ' => vec![(InputButton::H, Dire::U), (InputButton::Point, Dire::C)],
        'プ' => vec![
            (InputButton::H, Dire::U),
            (InputButton::Point, Dire::C),
            (InputButton::Point, Dire::C),
        ],
        'ヘ' => vec![(InputButton::H, Dire::R)],
        'ベ' => vec![(InputButton::H, Dire::R), (InputButton::Point, Dire::C)],
        'ペ' => vec![
            (InputButton::H, Dire::R),
            (InputButton::Point, Dire::C),
            (InputButton::Point, Dire::C),
        ],
        'ホ' => vec![(InputButton::H, Dire::D)],
        'ボ' => vec![(InputButton::H, Dire::D), (InputButton::Point, Dire::C)],
        'ポ' => vec![
            (InputButton::H, Dire::D),
            (InputButton::Point, Dire::C),
            (InputButton::Point, Dire::C),
        ],
        'マ' => vec![(InputButton::M, Dire::C)],
        'ミ' => vec![(InputButton::M, Dire::L)],
        'ム' => vec![(InputButton::M, Dire::U)],
        'メ' => vec![(InputButton::M, Dire::R)],
        'モ' => vec![(InputButton::M, Dire::D)],
        'ャ' => vec![(InputButton::Y, Dire::C), (InputButton::Point, Dire::C)],
        'ヤ' => vec![(InputButton::Y, Dire::C)],
        'ュ' => vec![(InputButton::Y, Dire::U), (InputButton::Point, Dire::C)],
        'ユ' => vec![(InputButton::Y, Dire::U)],
        'ョ' => vec![(InputButton::Y, Dire::D), (InputButton::Point, Dire::C)],
        'ヨ' => vec![(InputButton::Y, Dire::D)],
        'ラ' => vec![(InputButton::R, Dire::C)],
        'リ' => vec![(InputButton::R, Dire::L)],
        'ル' => vec![(InputButton::R, Dire::U)],
        'レ' => vec![(InputButton::R, Dire::R)],
        'ロ' => vec![(InputButton::R, Dire::D)],
        'ヮ' => vec![(InputButton::W, Dire::C), (InputButton::Point, Dire::C)],
        'ワ' => vec![(InputButton::W, Dire::C)],
        'ヲ' => vec![(InputButton::W, Dire::L), (InputButton::Point, Dire::C)],
        'ン' => vec![(InputButton::W, Dire::U)],
        'ー' => vec![(InputButton::Line, Dire::C)],
        _ => return None,
    })
}

pub fn string_to_keys(s: &String) -> Option<Vec<(InputButton, Dire)>> {
    let mut last = None;
    let mut keys = Vec::new();
    for c in s.chars() {
        let ckeys = char_to_keys(c)?;
        if let Some(last) = &last {
            if &ckeys.first().unwrap().0 == last {
                keys.push((InputButton::Enter, Dire::C));
            }
        }
        for key in ckeys {
            last = Some(key.0.clone());
            keys.push(key);
        }
    }
    keys.push((InputButton::Send, Dire::C));
    Some(keys)
}
