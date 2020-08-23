use std::num::ParseIntError;

use nom::{
    bytes::complete::{take, take_while, take_while_m_n},
    character::complete::char,
    combinator::map_res,
    sequence::tuple,
    IResult,
};

use crate::object::constant::*;

#[derive(Debug, PartialEq)]
pub struct Tree {
    pub leaves: Vec<Leaf>,
}

impl Tree {
    pub fn serialize(&self) -> Vec<u8> {
        self.leaves.iter().flat_map(|l| l.serialize()).collect()
    }

    fn deserialize_leaves(bytes: &[u8]) -> Vec<Leaf> {
        if bytes.is_empty() {
            vec![]
        } else {
            let (remainder, leaf) = Leaf::deserialize(&bytes);
            let mut other = Self::deserialize_leaves(remainder);
            other.push(leaf);
            other
        }
    }

    pub fn deserialize(bytes: Vec<u8>) -> Self {
        Self {
            leaves: Self::deserialize_leaves(&bytes),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Leaf {
    mode: String,
    pub path: String,
    pub hash: String,
}

impl Leaf {
    fn encode_hash(hash: &str) -> Vec<u8> {
        hex::decode(hash).expect("Unable to encode hash.")
    }

    fn decode_hash(bytes: &[u8]) -> String {
        hex::encode(bytes)
    }

    pub fn serialize(&self) -> Vec<u8> {
        vec![
            self.mode.as_bytes(),
            &vec![ASCII_SPACE],
            self.path.as_bytes(),
            &vec![ASCII_NULL],
            &Self::encode_hash(&self.hash),
        ]
        .concat()
    }

    fn parse_mode(input: &[u8]) -> IResult<&[u8], String> {
        // NOTE[Rhys] modes don't have to be 6 characters as leading 0s are dropped
        map_res(take_while_m_n(0, 6, |c| c != ASCII_SPACE), |c: &[u8]| {
            String::from_utf8(c.to_vec())
        })(input)
    }

    fn parse_path(input: &[u8]) -> IResult<&[u8], String> {
        map_res(take_while(|c| c != ASCII_NULL), |c: &[u8]| {
            String::from_utf8(c.to_vec())
        })(input)
    }

    fn parse_hash(input: &[u8]) -> IResult<&[u8], String> {
        // Note[Rhys] this ParseIntError is a lie but nom expects there to be some error type here
        map_res(take(20 as usize), |h: &[u8]| {
            Ok::<String, ParseIntError>(Leaf::decode_hash(h))
        })(input)
    }

    fn deserialize(bytes: &[u8]) -> (&[u8], Self) {
        let (remainder, (mode, _, path, _, hash)) = tuple((
            Leaf::parse_mode,
            char(ASCII_SPACE_CHAR),
            Leaf::parse_path,
            char(ASCII_NULL_CHAR),
            Leaf::parse_hash,
        ))(bytes)
        .unwrap();

        (remainder, Self { mode, path, hash })
    }
}

#[cfg(test)]
mod tests {
    use crate::object::constant::{ASCII_NULL, ASCII_SPACE};
    use crate::object::tree::{Leaf, Tree};

    #[test]
    fn decodes_hash() {
        let raw = vec![
            0, 219, 250, 237, 236, 71, 165, 169, 35, 228, 150, 70, 108, 63, 223, 76, 200, 117, 247,
            74,
        ];
        let parsed = "00dbfaedec47a5a923e496466c3fdf4cc875f74a";
        assert_eq!(Leaf::decode_hash(&raw), parsed)
    }

    #[test]
    fn encodes_hash() {
        let raw = "00dbfaedec47a5a923e496466c3fdf4cc875f74a";
        let parsed = vec![
            0, 219, 250, 237, 236, 71, 165, 169, 35, 228, 150, 70, 108, 63, 223, 76, 200, 117, 247,
            74,
        ];
        assert_eq!(Leaf::encode_hash(&raw), parsed)
    }

    #[test]
    fn parses_mode() {
        let raw = [49, 48, 48, 54, 52, 52, ASCII_SPACE];
        let (remainder, mode) = Leaf::parse_mode(&raw).unwrap();
        assert_eq!(mode, "100644");
        assert_eq!(remainder, [ASCII_SPACE]);
    }

    #[test]
    fn parses_mode_with_implicit_leading_zero() {
        let raw = [52, 48, 48, 48, 48, ASCII_SPACE];
        let (remainder, mode) = Leaf::parse_mode(&raw).unwrap();
        assert_eq!(mode, "40000");
        assert_eq!(remainder, [ASCII_SPACE]);
    }

    #[test]
    fn parses_path() {
        let raw = [82, 69, 65, 68, 77, 69, 46, 109, 100, ASCII_NULL];
        let (remainder, mode) = Leaf::parse_path(&raw).unwrap();
        assert_eq!(mode, "README.md");
        assert_eq!(remainder, [ASCII_NULL]);
    }

    #[test]
    fn parses_hash() {
        let expected_remainder: u8 = 100;
        let raw: Vec<u8> = vec![
            0,
            219,
            250,
            237,
            236,
            71,
            165,
            169,
            35,
            228,
            150,
            70,
            108,
            63,
            223,
            76,
            200,
            117,
            247,
            74,
            expected_remainder,
        ];
        let (remainder, mode) = Leaf::parse_hash(&raw).unwrap();
        assert_eq!(mode, "00dbfaedec47a5a923e496466c3fdf4cc875f74a");
        assert_eq!(remainder, [expected_remainder]);
    }

    #[test]
    fn round_trips_leaf() {
        let mode: Vec<u8> = vec![49, 48, 48, 54, 52, 52];
        let path: Vec<u8> = vec![46, 103, 105, 116, 105, 103, 110, 111, 114, 101];
        let hash: Vec<u8> = vec![
            234, 140, 75, 247, 243, 95, 111, 119, 247, 93, 146, 173, 140, 232, 52, 159, 110, 129,
            221, 186,
        ];
        let serialized: &[u8] = &[
            mode.as_slice(),
            &[ASCII_SPACE],
            path.as_slice(),
            &[ASCII_NULL],
            hash.as_slice(),
        ]
        .concat();

        let deserialized = Leaf {
            mode: "100644".to_string(),
            path: ".gitignore".to_string(),
            hash: "ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba".to_string(),
        };
        let (remainder, leaf) = Leaf::deserialize(&serialized);
        assert!(remainder.is_empty());
        assert_eq!(leaf, deserialized);
        assert_eq!(leaf.serialize(), serialized)
    }

    #[test]
    fn round_trips_empty_tree() {
        let empty = Tree { leaves: vec![] };
        let tree = Tree::deserialize(vec![]);
        assert_eq!(tree, empty);
        assert_eq!(tree.serialize(), vec![]);
    }
}
