use aes::block_cipher::generic_array::GenericArray;
use aes::block_cipher::{BlockCipher, NewBlockCipher};
use aes::Aes128;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Label {
    pub len: u16,
    pub label: u64,
}

impl Label {
    pub fn empty() -> Self {
        Label { len: 0, label: 0 }
    }

    pub fn from(label_in: u64) -> Self {
        Label {
            len: 64,
            label: label_in,
        }
    }

    pub fn new(label_in: u64, len_in: u16) -> Self {
        let shifted = label_in << (64 - len_in);
        Label {
            len: len_in,
            label: shifted,
        }
    }

    pub fn to(&self) -> u64 {
        self.label >> (64 - self.len)
    }

    pub fn get_bit(&self, index: i32) -> u8 {
        ((self.label >> (63 - index)) & 1) as u8
    }

    pub fn set_bit(&mut self, index: i32) {
        self.label = self.label | (1u64 << (63 - index))
    }

    pub fn is_prefix(&self, other: &Label) -> bool {
        if self.len > other.len {
            false
        } else {
            self.len == 0 || (self.label ^ (other.label)) >> (64 - self.len) == 0
        }
    }

    pub fn reduce_len(&mut self, len: u16) {
        if self.len - len <= 0 {
            self.len = 0;
            self.label = 0;
        } else {
            self.len -= len;
            self.label = self.to() << (64 - self.len);
        }
    }

    pub fn reduce_sub(&mut self) {
        self.reduce_len(1);
        if self.len > 0 {
            self.label -= 1u64 << (64 - self.len);
        }
    }

    pub fn reduce_add(&mut self) {
        self.reduce_len(1);
        if self.len > 0 {
            self.label += 1u64 << (64 - self.len);
        }
    }
}

impl Ord for Label {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = (64 - self.len).cmp(&(64 - other.len));
        if order == Ordering::Equal {
            self.label.cmp(&other.label)
        } else {
            order
        }
    }
}

impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Label {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.len == other.len
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(label: {}, length: {})", self.label, self.len)
    }
}

#[derive(Debug, Eq)]
pub struct Node {
    pub label: Label,
    pub key: [u8; 16],
}

pub fn apply_aes(key: &[u8; 16], label: &Label, from: i32, to: i32, out_key: &mut [u8; 16]) {
    let len = to - from;
    let mut block = GenericArray::clone_from_slice(&[0u8; 16]);
    let mut key_aes = GenericArray::clone_from_slice(key);

    for offset in 0..len {
        //println!("{:?}", label.get_bit(from + offset));
        block[15] = label.get_bit(from + offset);
        let cipher = Aes128::new(&key_aes);
        cipher.encrypt_block(&mut block);
        key_aes.copy_from_slice(&block);
        block.iter_mut().for_each(|x| *x = 0)
    }
    out_key.clone_from_slice(key_aes.as_slice());
}

impl Node {
    pub fn master_node(master_key: [u8; 16]) -> Self {
        Node {
            label: Label { len: 0, label: 0 },
            key: master_key,
        }
    }

    pub fn compute_key(&self, label: &Label, out_key: &mut [u8; 16]) -> bool {
        if self.label.is_prefix(label) {
            apply_aes(
                &self.key,
                label,
                self.label.len as i32,
                label.len as i32,
                out_key,
            );
            true
        } else {
            false
        }
    }

    pub fn compute_node(&self, label: &Label) -> Option<Node> {
        if self.label.is_prefix(label) {
            let mut node = Node {
                label: *label,
                key: [0u8; 16],
            };
            apply_aes(
                &self.key,
                label,
                self.label.len as i32,
                label.len as i32,
                &mut node.key,
            );
            Some(node)
        } else {
            None
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.label.cmp(&other.label)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(Node label: {}, key: {:?})", self.label, self.key)
    }
}
