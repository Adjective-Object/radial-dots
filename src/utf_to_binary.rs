pub fn text_to_binary(input: &str) -> Option<Vec<bool>> {
    let mut cur = [0];
    let mut result: Vec<bool> = Vec::with_capacity(input.len() * 8);
    for c in input.chars() {
        if !c.is_ascii() {
            return None;
        }
        c.encode_utf8(&mut cur);

        for i in 0..8 {
            result.push((1 << i) & i != 0);
        }
    }
    return Some(result);
}
