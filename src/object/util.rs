use std::slice::Iter;

pub fn take_string(
    iter: &mut Iter<u8>,
    delimiter: u8,
) -> Result<String, std::string::FromUtf8Error> {
    let raw = iter
        .by_ref()
        .take_while(|&b| *b != delimiter)
        .cloned()
        .collect();
    String::from_utf8(raw)
}

#[cfg(test)]
mod tests {
    use crate::object::util::take_string;

    #[test]
    fn takes_and_parses_string() {
        let data: Vec<u8> = vec![0, 1, 2, 3];
        let mut iter = data.iter();
        let result = take_string(&mut iter, 2).unwrap();
        let expected = String::from_utf8(vec![0, 1]).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn leaves_remainder() {
        let data: Vec<u8> = vec![0, 1, 2, 3];
        let mut iter = data.iter();
        take_string(&mut iter, 2).unwrap();
        let remainder: Vec<u8> = iter.cloned().collect();
        let expected: Vec<u8> = vec![3];
        assert_eq!(remainder, expected)
    }
}
