use std::error::Error;

use crate::object::constant::*;
use crate::object::util::take_string;

#[derive(Debug)]
pub struct Tree {
    leaves: Vec<Leaf>,
}

impl Tree {
    pub fn serialize(&self) -> Vec<u8> {
        self.leaves.iter().flat_map(|l| l.serialize()).collect()
    }

    pub fn deserialize(body: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut remainder = body;
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

    fn decode_hash(raw: &Vec<u8>) -> String {
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

    pub fn deserialize(body: Vec<u8>) -> Result<(Self, Vec<u8>), Box<dyn Error>> {
        let mut iter = body.iter();
        let mode = take_string(&mut iter, ASCII_SPACE)?;
        let path = take_string(&mut iter, ASCII_NULL)?;
        let hash_raw: Vec<u8> = iter.by_ref().take(20).cloned().collect();
        let hash = Self::decode_hash(&hash_raw);
        let remainder: Vec<u8> = iter.cloned().collect();

        Ok((Self { mode, path, hash }, remainder))
    }
}

#[cfg(test)]
mod tests {
    use crate::object::constant::{ASCII_NULL, ASCII_SPACE};
    use crate::object::tree::Leaf;

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
        let (leaf, remainder) = Leaf::deserialize(serialized.clone()).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(leaf, deserialized);
        assert_eq!(leaf.serialize(), serialized)
    }
}
