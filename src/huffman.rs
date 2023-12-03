use std::collections::HashMap;
use std::fs::File;
use std::hash::BuildHasher;
use std::io::{self, Read, BufWriter, BufReader, Write};
use std::path::Path;
use std::thread::{self, available_parallelism};

use crate::fileops;
use crate::map::{ByteMap, CountMap};
use crate::heap::Heap;
use crate::node::Node;

use concat_strs::concat_strs;
use itertools::Itertools;

const HASH_MAP_CAPACITY: usize = 256;
const BYTES_MULTIPLIER: usize = 80_000;
const READER_CAPACITY: usize = 10_485_760;      // Probably can't fit in stack

pub struct Huffman;

impl Huffman {
    // Builds a huffman tree from a heap of nodes
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

    pub fn encode(file_path: &Path, new_path: &Path) {
        let file = File::open(file_path).unwrap();
        let new_file = fileops::create_file(new_path).unwrap();

        let mut reader = BufReader::new(&file).bytes();
        let mut writer = BufWriter::new(&new_file);

        let mut heap = Huffman::create_node_heap(file_path);
        let root = Huffman::build_tree(&mut heap);
        let table = Self::generate_codes(&root);

        while let Some(Ok(byte)) = reader.next() {
            let mut bit_num = 7;
            let mut output: u8 = 0b0000_0000;

            if let Some(code) = table.get(byte) {
                for bit in code.chars() {
                    match bit {
                        '0' => {
                            output |= (0 << bit_num);
                        },
                        '1' => {
                            output |= (1 << bit_num);
                        },
                        _ => (),
                    }
                    if bit_num > 0 { 
                        bit_num -= 1; 
                    } else if bit_num == 0 { 
                        writer.write_all(&[output]);
                        output = 0b0000_0000; 
                    }
                }
            };
        }
    }

    // Generate huffman codes recursively from a huffman tree
    fn generate_codes(root: &Node) -> ByteMap<String> {
        let mut map = ByteMap::new();
        Self::gen_codes_helper(&mut map, Some(root), "");
        map
    }

    // Helper for generating huffman codes from a huffman tree
    fn gen_codes_helper(table: &mut ByteMap<String>, curr_node: Option<&Node>, curr_code: &str) {
        if let Some(node) = curr_node {
            if node.get_left() == None && node.get_right() == None {
                table.insert(curr_code.to_string(), node.get_byte().unwrap());
            } else {
                Self::gen_codes_helper(table, node.get_left(), &concat_strs!(curr_code, '0'));
                Self::gen_codes_helper(table, node.get_right(), &concat_strs!(curr_code, '1'));
            }
        }
    }

    // Does the same thing as generate_codes but uses a stack instead of recursion
    // to prevent stack overflow
    fn gen_codes__safe(root: &Node) -> ByteMap<String> {
        let mut stack = Vec::with_capacity(256);
        let mut table = ByteMap::new();
        stack.push((root, String::new()));

        while let Some((node, code)) = stack.pop() {
            let code: &str = code.as_ref();
            if node.get_left() == None && node.get_right() == None {
                table.insert(code.to_string(), node.get_byte().unwrap());
            } else {
                if let Some(left_node) = node.get_left() {
                    stack.push((left_node, concat_strs!(code, '0')));
                } 
                if let Some(right_node) = node.get_right() {
                    stack.push((right_node, concat_strs!(code, '1')));
                }
            }
        }
        table
    }

    // Creates a heap of nodes from a file
    fn create_node_heap(file_path: &Path) -> Heap<Node> {
        let file = File::open(file_path).unwrap();
        let char_count = Self::create_freq_table(&file);

        let mut node_vec: Vec<Node> = Vec::with_capacity(256);
        for (byte, count) in char_count {
            node_vec.push(Node::new(byte, count));
        }
        Heap::new(node_vec)
    } 

    pub fn create_freq_table(file: &File) -> CountMap {
        let worker_count = available_parallelism().unwrap().get();
        let bytes_each = worker_count * BYTES_MULTIPLIER;

        let reader = io::BufReader::with_capacity(READER_CAPACITY, file);
        let mut result = CountMap::new();

        for page in &reader.bytes().chunks(bytes_each) {
            let data: Vec<_> = page.filter_map(Result::ok).collect();

            match data.len() {
                0 => (),
                l if l < 320_000 => count_single_thread(&mut result, data),
                _ => count_multi_thread(&mut result, data, worker_count)
            }
        }

        result
    }

}

#[inline]
fn count_single_thread(result: &mut CountMap, data: Vec<u8>) {
    for byte in data {
        result.increment_count(byte);
    }
}

#[inline]
fn count_multi_thread(result: &mut CountMap, data: Vec<u8>, threads: usize) {
    thread::scope(|s| {
        let mut handles = Vec::with_capacity(threads);
        for chunk in data.chunks(data.len()/ threads + 1) {
            handles.push(s.spawn( || count(chunk)));
        }
        for handle in handles {
            let map = handle.join().unwrap();
            for (key, value) in map {
                result.increment_count_by(key, value);
            }
        }
    })
}

#[inline]
fn count(input: &[u8]) -> CountMap {
    let mut map = CountMap::new();
    for byte in input {
        map.increment_count(*byte);
    }
    map
}