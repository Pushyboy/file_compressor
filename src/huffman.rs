use std::collections::HashMap;
use std::io::{self, BufRead, Read};
use std::path::Path;
use std::thread;

use crate::fnv::FNV1aBuilder;
use crate::heap::Heap;
use crate::node::Node;
use crate::io::IO;

use itertools::Itertools;

pub struct Huffman {

}

impl Huffman {
    /// Builds a huffman tree from a heap and returns
    /// the head of the tree
    fn build_tree(heap: &mut Heap<Node>) -> Node {
        let size = heap.size();

        for _ in 1..size {
            let left = heap.remove().unwrap();
            let right = heap.remove().unwrap();
            let freq = left.get_freq() + right.get_freq();

            let node = Node::new_huffman(left, right, freq);
            heap.insert(node);
        }

        heap.remove().unwrap()
    }

    fn create_freq_file(file_path: &Path, freq_path: &Path) {
        let file = IO::create_file(file_path).unwrap();
        let mut reader = io::BufReader::new(file);

        let mut result: HashMap<char, usize, _> = HashMap::with_hasher(FNV1aBuilder);

        // let worker_threads: usize = 4;
        // let mut handles = Vec::new();

        for page in &reader.lines().chunks(4000) {
            let string: Vec<_> = page.filter_map(Result::ok).collect();

            match string.len() {
                0 => HashMap::with_hasher(FNV1aBuilder),
                n if n < 4000 => Self::count_chars(string),
                _ => {

                }

            }

            // let string: Vec<_> = page.filter_map(Result::ok).collect();
            
            // for chunk in string.chunks(worker_threads) {
            //     let string = chunk.join("");
                
            //     let handle = thread::spawn(move || {
            //         let mut map: HashMap<char, usize, _> = HashMap::with_hasher(FNV1aBuilder);
            //         for c in string.chars() {
            //             *map.entry(c).or_default() += 1;
            //         }

            //         map
            //     });

            //     handles.push(handle)
            // }
        }

        // for handle in handles {
        //     let map = handle.join().unwrap();
        //     for (key, value) in map {
        //         *result.entry(key).or_default() += value;
        //     }
        // }
        
        // *map.entry(key)
    }

    pub fn count_chars(input: Vec<String>) -> HashMap<char, usize, FNV1aBuilder> {

    }
}