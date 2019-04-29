pub fn text_to_binary(input: &str) -> Option<Vec<bool>> {
    let mut cur = [0];
    let mut result: Vec<bool> = Vec::with_capacity(input.len() * 8);
    for c in input.chars() {
        if !c.is_ascii() {
            return None;
        }
        c.encode_utf8(&mut cur);

        for i in 0..8 {
            result.push((1 << (7 - i)) & cur[0] != 0);
        }
    }
    return Some(result);
}

#[cfg(test)]
mod tests {
    use crate::utf_to_binary::text_to_binary;

    #[test]
    fn test_single_character() {
        assert_eq!(
            text_to_binary("a"),
            Some(vec![false, true, true, false, false, false, false, true]),
        );
    }

    #[test]
    fn test_2_chars() {
        assert_eq!(
            text_to_binary("ayy"),
            Some(vec![
                // 'a'
                false, true, true, false, false, false, false, true, // 'y'
                false, true, true, true, true, false, false, true, // 'y'
                false, true, true, true, true, false, false, true,
            ]),
        );
    }

}
