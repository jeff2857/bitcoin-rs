use std::fmt::Display;

use hex::ToHex;

use num_traits::Pow;

pub struct MerkleTree {
    pub total: usize,
    pub max_depth: usize,
    pub nodes: Vec<Vec<Option<Vec<u8>>>>,
    current_depth: usize,
    current_index: usize,
}

impl MerkleTree {
    pub fn new(total: usize) -> Self {
        let max_depth = total.log2() as usize;
        let mut nodes = vec![];
        for depth in 0..(max_depth + 1) {
            let num_items = total / 2usize.pow((max_depth - depth) as u32);
            let mut level_hashes = vec![];
            for _ in 0..num_items {
                level_hashes.push(None);
            }
            nodes.push(level_hashes);
        }

        Self {
            total,
            max_depth,
            nodes,
            current_index: 0,
            current_depth: 0,
        }
    }
}

impl Display for MerkleTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = vec![];
        for (depth, level) in self.nodes.clone().into_iter().enumerate() {
            let mut items: Vec<String> = vec![];
            for (index, h) in level.into_iter().enumerate() {
                let short;
                if h.is_none() {
                    short = "None".to_string();
                } else {
                    let h_bytes = h.as_ref().unwrap();
                    short = format!("{}...", &h_bytes.encode_hex::<String>()[0..8]);
                }

                if depth == self.current_depth && index == self.current_index {
                    items.push(format!("*{}*", &short[0..(short.len() - 2)]));
                } else {
                    items.push(short);
                }
            }

            result.push(items.join(", "));
        }

        let result = result.join("\n");

        write!(f, "{}", result)
    }
}


#[cfg(test)]
mod tests_merkle_tree {
    #[test]
    fn test_construct() {

    }
}
