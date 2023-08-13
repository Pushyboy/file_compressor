use std::{cmp::Ordering};

pub struct Node {
    left: Box<Option<Node>>,
    right: Box<Option<Node>>,
    char: Option<char>,
    freq: usize,
}

impl Node {
    pub fn new(char: char, freq: usize) -> Node {
        Node {
            left: Box::new(None),
            right: Box::new(None),
            char: Some(char),
            freq
        }
    }

    pub fn new_huffman(left: Node, right: Node, freq: usize) -> Node {
        Node {
            left: Box::new(Some(left)),
            right: Box::new(Some(right)),
            char: None,
            freq
        }
    }

    pub fn get_freq(&self) -> usize {
        self.freq
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
        }
        else if self.freq == other.freq {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    }
}


