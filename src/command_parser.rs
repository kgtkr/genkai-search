fn tokenize_input(line: String) -> Vec<String> {
    line.trim_end_matches("\n")
        .split(' ')
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
}

fn satisfy<T>(items: Vec<T>, f: impl FnOnce(&T) -> bool) -> (Option<T>, Vec<T>) {
    let mut iters = items.into_iter();
    if let Some(first) = iters.next() {
        if f(&first) {
            (Some(first), iters.collect())
        } else {
            (None, std::iter::once(first).chain(iters).collect())
        }
    } else {
        (None, iters.collect())
    }
}

pub fn parse_command(line: String) -> (Option<String>, Vec<String>) {
    let tokens = tokenize_input(line);
    let (cmd, params) = satisfy(tokens, |x| x.chars().next() == Some(':'));
    (cmd.map(|x| x.chars().skip(1).collect()), params)
}
