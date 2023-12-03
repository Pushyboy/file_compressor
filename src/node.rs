use std::{cmp::Ordering, fmt::Display};

pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    byte: Option<u8>,
    freq: usize,
}

impl Node {
    /// Generates a new node
    pub fn new(byte: u8, freq: usize) -> Node {
        Node {
            left: None,
            right: None,
            byte: Some(byte),
            freq
        }
    }

    /// Generates a new huffman node
    pub fn new_huffman(left: Node, right: Node, freq: usize) -> Node {
        Node {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            byte: None,
            freq
        }
    }

    pub fn get_freq(&self) -> usize {
        self.freq
    }

    pub fn get_byte(&self) -> Option<u8> {
        self.byte
    }

    pub fn get_left(&self) -> Option<&Node> {
        self.left.as_deref()
    }

    pub fn get_right(&self) -> Option<&Node> {
        self.right.as_deref()
    }

    pub fn set_left(&mut self, n: Node) {
        self.left = Some(Box::new(n));
    }

    pub fn set_right(&mut self, n: Node) {
        self.right = Some(Box::new(n));
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.freq < other.freq {
            Some(Ordering::Greater)
        } else if self.freq == other.freq {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"[{}: {}]", self.byte.unwrap_or_default(), self.freq)
    }
}


