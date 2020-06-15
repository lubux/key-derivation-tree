use crate::tree::{Label, Node};
use crate::errors::InvalidAccessError;

pub struct ConstrainedPrf {
    pub label_bits: u16,
    pub nodes: Vec<Node>,
}

impl ConstrainedPrf {
    pub fn new(input_bits: u16, nodes_in: Vec<Node>) -> Self {
        ConstrainedPrf {
            label_bits: input_bits,
            nodes: nodes_in,
        }
    }

    pub fn init(input_bits: u16, key: [u8; 16]) -> Self {
        ConstrainedPrf {
            label_bits: input_bits,
            nodes: vec![Node {
                label: Label::empty(),
                key: key,
            }],
        }
    }

    fn find_node(&self, label: &Label) -> Option<&Node> {
        for n in self.nodes.iter().rev() {
            if n.label.is_prefix(label) {
                return Some(n);
            }
        }
        None
    }

    pub fn apply(&self, input: u64) -> Result<[u8; 16], InvalidAccessError> {
        let label = Label::new(input, self.label_bits);
        let node = self.find_node(&label);
        if let Some(n) = node {
            let mut result = [0u8; 16];
            if n.compute_key(&label, &mut result) {
                Ok(result)
            } else {
                Err(InvalidAccessError{key_id: input})
            }
        } else {
            Err(InvalidAccessError{key_id: input})
        }
    }

    fn derive_node(&self, label: &Label) -> Option<Node> {
        let node = self.find_node(label);
        if let Some(n) = node {
            n.compute_node(&label)
        } else {
            None
        }
    }

    pub fn constrain(&self, from: u64, to: u64) -> Result<Vec<Node>, &str> {
        debug_assert!(to >= from);
        let mut result = Vec::new();
        let mut label_from = Label::new(from, self.label_bits);
        let mut label_to = Label::new(to - 1, self.label_bits);
        let mut pow = 0;
        while label_from < label_to {
            //println!("From {}, To {}", label_from, label_to);
            // FROM
            let bit = label_from.get_bit((self.label_bits - pow - 1) as i32);
            if bit == 1 {
                if let Some(n) = self.derive_node(&label_from) {
                    result.push(n);
                } else {
                    return Err("Failed");
                }
                label_from.reduce_add();
            } else {
                label_from.reduce_len(1);
            }
            // TO
            //println!("BITS {}", self.label_bits - pow - 1);
            let bit = label_to.get_bit((self.label_bits - pow - 1) as i32);
            if bit == 1 {
                label_to.reduce_len(1);
            } else {
                if let Some(n) = self.derive_node(&label_to) {
                    result.push(n);
                } else {
                    return Err("Failed");
                }
                label_to.reduce_sub();
            }
            pow += 1;
        }
        //println!("From {}, To {}", label_from, label_to);
        if label_from == label_to {
            if let Some(n) = self.derive_node(&label_to) {
                result.push(n);
            } else {
                return Err("Failed");
            }
        }
        result.sort_by(|b, a| a.cmp(b));
        Ok(result)
    }
}


