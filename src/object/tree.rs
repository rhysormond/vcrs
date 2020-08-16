use std::error::Error;
use std::num::ParseIntError;

use nom::{
    bytes::complete::{take, take_while, take_while_m_n},
    combinator::map_res,
    IResult,
};

use crate::object::constant::*;

#[derive(Debug, PartialEq)]
pub struct Tree {
    leaves: Vec<Leaf>,
}

impl Tree {
    pub fn serialize(&self) -> Vec<u8> {
        self.leaves.iter().flat_map(|l| l.serialize()).collect()
    }

    pub fn deserialize(body: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut remainder: &[u8] = body.as_slice();
        let mut leaves: Vec<Leaf> = vec![];
        while !remainder.is_empty() {
            let (leaf, rest) = Leaf::deserialize(remainder)?;
            leaves.push(leaf);
            remainder = rest
        }
        Ok(Self { leaves })
    }
}

#[derive(Debug, PartialEq)]
struct Leaf {
    mode: String,
    path: String,
    hash: String,
}

impl Leaf {
    fn encode_hash(raw: &String) -> Vec<u8> {
        hex::decode(raw).expect("Unable to encode hash.")
    }

    fn decode_hash(raw: &[u8]) -> String {
        hex::encode(raw)
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

    fn deserialize_mode(input: &[u8]) -> IResult<&[u8], String> {
        // NOTE[Rhys] modes don't have to be 6 characters as leading 0s are dropped
        map_res(take_while_m_n(0, 6, |c| c != ASCII_SPACE), |c: &[u8]| {
            String::from_utf8(c.to_vec())
        })(input)
    }

    fn deserialize_path(input: &[u8]) -> IResult<&[u8], String> {
        map_res(take_while(|c| c != ASCII_NULL), |c: &[u8]| {
            String::from_utf8(c.to_vec())
        })(input)
    }

    fn deserialize_hash(input: &[u8]) -> IResult<&[u8], String> {
        // Note[Rhys] this ParseIntError is a lie but nom expects there to be some error type here
        map_res(take(20 as usize), |h: &[u8]| {
            Ok::<String, ParseIntError>(Leaf::decode_hash(h))
        })(input)
    }

    pub fn deserialize(body: &[u8]) -> Result<(Self, &[u8]), Box<dyn Error>> {
        let remainder = body;
        let (remainder, mode) = Leaf::deserialize_mode(remainder).unwrap();
        // Note[Rhys] we slice remainder here to drop the space delimiter
        let (remainder, path) = Leaf::deserialize_path(&remainder[1..]).unwrap();
        // Note[Rhys] we slice remainder here to drop the null delimiter
        let (remainder, hash) = Leaf::deserialize_hash(&remainder[1..]).unwrap();
        Ok((Self { mode, path, hash }, remainder))
    }
}

#[cfg(test)]
mod tests {
    use crate::object::constant::{ASCII_NULL, ASCII_SPACE};
    use crate::object::tree::{Leaf, Tree};

    #[test]
    fn decodes_hash() {
        let raw: Vec<u8> = vec![
            0, 219, 250, 237, 236, 71, 165, 169, 35, 228, 150, 70, 108, 63, 223, 76, 200, 117, 247,
            74,
        ];
        let parsed: String = String::from("00dbfaedec47a5a923e496466c3fdf4cc875f74a");
        assert_eq!(Leaf::decode_hash(&raw), parsed)
    }

    #[test]
    fn encodes_hash() {
        let raw: String = String::from("00dbfaedec47a5a923e496466c3fdf4cc875f74a");
        let parsed: Vec<u8> = vec![
            0, 219, 250, 237, 236, 71, 165, 169, 35, 228, 150, 70, 108, 63, 223, 76, 200, 117, 247,
            74,
        ];
        assert_eq!(Leaf::encode_hash(&raw), parsed)
    }

    #[test]
    fn deserializes_mode() {
        let raw = [49, 48, 48, 54, 52, 52, ASCII_SPACE];
        let (remainder, mode) = Leaf::deserialize_mode(&raw).unwrap();
        assert_eq!(mode, "100644");
        assert_eq!(remainder, [ASCII_SPACE]);
    }

    #[test]
    fn deserializes_mode_with_implicit_leading_zero() {
        let raw = [52, 48, 48, 48, 48, ASCII_SPACE];
        let (remainder, mode) = Leaf::deserialize_mode(&raw).unwrap();
        assert_eq!(mode, "40000");
        assert_eq!(remainder, [ASCII_SPACE]);
    }

    #[test]
    fn deserializes_path() {
        let raw = [82, 69, 65, 68, 77, 69, 46, 109, 100, ASCII_NULL];
        let (remainder, mode) = Leaf::deserialize_path(&raw).unwrap();
        assert_eq!(mode, "README.md");
        assert_eq!(remainder, [ASCII_NULL]);
    }

    #[test]
    fn deserializes_hash() {
        let test_char: u8 = 100;
        let raw: Vec<u8> = vec![
            0, 219, 250, 237, 236, 71, 165, 169, 35, 228, 150, 70, 108, 63, 223, 76, 200, 117, 247,
            74, test_char,
        ];
        let (remainder, mode) = Leaf::deserialize_hash(&raw).unwrap();
        assert_eq!(mode, "00dbfaedec47a5a923e496466c3fdf4cc875f74a");
        assert_eq!(remainder, [test_char]);
    }

    #[test]
    fn round_trips_leaf() {
        let mode: Vec<u8> = vec![49, 48, 48, 54, 52, 52];
        let path: Vec<u8> = vec![46, 103, 105, 116, 105, 103, 110, 111, 114, 101];
        let hash: Vec<u8> = vec![
            234, 140, 75, 247, 243, 95, 111, 119, 247, 93, 146, 173, 140, 232, 52, 159, 110, 129,
            221, 186,
        ];
        let serialized: Vec<u8> = vec![mode, vec![ASCII_SPACE], path, vec![ASCII_NULL], hash]
            .iter()
            .flatten()
            .cloned()
            .collect();
        let deserialized = Leaf {
            mode: String::from("100644"),
            path: String::from(".gitignore"),
            hash: String::from("ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba"),
        };
        let (leaf, remainder) = Leaf::deserialize(&serialized).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(leaf, deserialized);
        assert_eq!(leaf.serialize(), serialized)
    }

    #[test]
    fn round_trips_empty_tree() {
        let empty = Tree { leaves: vec![] };
        let tree = Tree::deserialize(vec![]).unwrap();
        assert_eq!(tree, empty);
        assert_eq!(tree.serialize(), vec![]);
    }
}
