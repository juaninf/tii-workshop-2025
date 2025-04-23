use bitvec::prelude::*;
use ndarray::Array2;
use numpy::{PyArray2, ToPyArray};
use pyo3::prelude::*;
use pyo3::pyfunction;
use pyo3::types::PyList;

use bitvec::prelude::{BitVec, Msb0};
use pyo3::prelude::*; // for Python<'py> // for BitVec and Msb0

pub fn sbox_component(input: BitVec<u8, Msb0>, lookup_table: &[u64]) -> BitVec<u8, Msb0> {
    let value: u64 = lookup_table[input.load_be::<usize>()];
    let output_len = std::cmp::max(1, 64 - value.leading_zeros()) as usize;
    let mut bv = bitvec![u8, Msb0; 0; output_len];
    bv[0..output_len].store(value);
    bv
}

pub fn xor_component(inputs: &[BitVec<u8, Msb0>]) -> BitVec<u8, Msb0> {
    assert!(!inputs.is_empty(), "Input list cannot be empty");

    let bit_len = inputs[0].len();
    for bv in inputs {
        assert_eq!(bv.len(), bit_len, "All inputs must be the same length");
    }

    inputs[1..]
        .iter()
        .cloned()
        .fold(inputs[0].clone(), |acc, bv| acc ^ bv)
}

pub fn linear_layer_component(
    input: &BitVec<u8, Msb0>,
    linear_layer_matrix: &Array2<u64>,
    transpose: bool,
) -> BitVec<u8, Msb0> {
    assert!(input.len() <= 64, "Input must be <= 64 bits");

    let mut padded = bitvec![u8, Msb0; 0; 64];
    let offset = 64 - input.len();
    padded[offset..].copy_from_bitslice(input);
    let input_val: u64 = padded.load_be::<u64>();

    let matrix = if transpose {
        linear_layer_matrix.t()
    } else {
        linear_layer_matrix.view()
    };

    let mut result = BitVec::<u8, Msb0>::with_capacity(matrix.nrows());

    for row in matrix.rows() {
        let row_mask = row.iter().fold(0u64, |acc, &bit| (acc << 1) | (bit & 1));
        let parity = (row_mask & input_val).count_ones() % 2;
        result.push(parity == 1);
    }

    result
}

pub fn rotate_component(input: &BitVec<u8, Msb0>, rotation_amount: isize) -> BitVec<u8, Msb0> {
    let len = input.len();
    assert!(len > 0, "Input cannot be empty");

    let mut output = input.clone();

    if rotation_amount >= 0 {
        output.rotate_right((rotation_amount as usize) % len);
    } else {
        output.rotate_left((-rotation_amount as usize) % len);
    }

    output
}

pub fn constant_component(value: u64, length: usize) -> BitVec<u8, Msb0> {
    assert!(length <= 64, "Length must be ≤ 64 bits");

    let mut bits = bitvec![u8, Msb0; 0; length];
    bits[..].store_be(value);
    bits
}

#[pyfunction]
pub fn py_sbox_component(input: Vec<bool>, table: Vec<u64>) -> Vec<bool> {
    let bv = BitVec::<u8, Msb0>::from_iter(input);
    sbox_component(bv, &table).iter().copied().collect()
}

#[pyfunction]
pub fn py_xor_component(inputs: Vec<Vec<bool>>) -> Vec<bool> {
    let bvs: Vec<BitVec<u8, Msb0>> = inputs.into_iter().map(BitVec::from_iter).collect();
    xor_component(&bvs).iter().copied().collect()
}

#[pyfunction]
pub fn py_rotate_component(input: Vec<bool>, rotation_amount: isize) -> Vec<bool> {
    let bv = BitVec::<u8, Msb0>::from_iter(input);
    rotate_component(&bv, rotation_amount)
        .iter()
        .copied()
        .collect()
}

#[pyfunction]
pub fn py_constant_component(value: u64, length: usize) -> Vec<bool> {
    constant_component(value, length).iter().copied().collect()
}

#[pyfunction]
pub fn py_linear_layer_component<'py>(
    py: Python<'py>,
    input: Vec<bool>,
    matrix: &'py PyArray2<u64>,
    transpose: bool,
) -> Vec<bool> {
    let bv = BitVec::<u8, Msb0>::from_iter(input);
    let mat = unsafe { matrix.as_array() }; // safe here if passed correctly from Python
    linear_layer_component(&bv, &mat.to_owned(), transpose)
        .iter()
        .copied()
        .collect()
}

#[cfg(test)]
mod operator_tests;
