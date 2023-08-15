use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use std::thread::{self, available_parallelism};

use crate::fnv::FNV1aBuilder;
use crate::heap::Heap;
use crate::node::Node;
use crate::io::{IO};

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

    fn create_freq_file(file_path: &Path, freq_path: &Path) -> Vec<Node> {
        let freq_file = IO::create_file(freq_path).unwrap();
        let mut file = File::open(file_path).unwrap();
        let char_count = Self::create_freq_table(&file);

        let mut node_vec: Vec<Node> = Vec::with_capacity(char_count.len());

        for (char, count) in char_count {
            node_vec.push(Node::new(char, count));
            writeln!(&file, "{}:{}", char, count);
        }

        node_vec
    }

    fn create_freq_table(file: &File) -> HashMap<char, usize, FNV1aBuilder> {
        let worker_count: usize = available_parallelism().unwrap().get();
        let lines_each = worker_count * 1000;

        let reader = io::BufReader::with_capacity(10485760, file);
        let mut result: HashMap<char, usize, _> = HashMap::with_capacity_and_hasher(512, FNV1aBuilder);

        for page in &reader.lines().chunks(lines_each) {
            let string: Vec<_> = page.filter_map(Result::ok).collect();

            match string.len() {
                0 => (),
                l if l < 4000 => {
                    for line in string {
                        for c in line.chars() {
                            *result.entry(c).or_default() += 1;
                        }
                    }
                },
                l => thread::scope(|s| {
                    let mut handles = Vec::with_capacity(worker_count);
                    for chunk in string.chunks(l / worker_count + 1) {
                        handles.push(s.spawn(|| count_chars(chunk)))
                    }
                    for handle in handles {
                        let map = handle.join().unwrap();
                        for (key, value) in map {
                            *result.entry(key).or_default() += value;
                        }
                    }
                })
            };
        }

        result
    }



}

pub fn count_chars(input: &[String]) -> HashMap<char, usize, FNV1aBuilder> {
    let mut map = HashMap::with_capacity_and_hasher(512, FNV1aBuilder);
    for line in input {
        for c in line.chars() {
            *map.entry(c).or_default() += 1;
        }
    }
    map
}