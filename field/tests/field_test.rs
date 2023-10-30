use ark_std::{end_timer, start_timer};

use crate::goldilocks_field::GoldilocksField;
use crate::ops::Square;
use crate::packed::PackedField;
use crate::types::Field;
const ROUND: usize = 1 << 30;

#[test]
fn test_cpu_add() {
    let a_arr = test_vals_a();
    let b_arr = test_vals_b();

    let a = GoldilocksField::from_noncanonical_u64(2028722748960791447);
    let b = GoldilocksField::from_noncanonical_u64(2028722748960792447);

    let start = start_timer!(|| "cpu2_square");
    for _ in 0..ROUND * 4 {
        a + b
    }
    end_timer!(start);

    let start = start_timer!(|| "cpu512_square");
    for _ in 0..ROUND * 8 {
        a * b
    }
    end_timer!(start);
}

#[test]
fn test_cpu_mul() {
    let a_arr = test_vals_a();
    let b_arr = test_vals_b();

    let a = GoldilocksField::from_noncanonical_u64(2028722748960791447);
    let b = GoldilocksField::from_noncanonical_u64(2028722748960792447);

    let start = start_timer!(|| "cpu2_square");
    for _ in 0..ROUND * 4 {
        a * b
    }
    end_timer!(start);

    let start = start_timer!(|| "cpu512_square");
    for _ in 0..ROUND * 8 {
        a * b
    }
    end_timer!(start);
}

#[test]
fn test_cpu_square() {
    let a = GoldilocksField::from_noncanonical_u64(2028722748960791447);

    let start = start_timer!(|| "cpu2_square");
    for _ in 0..ROUND * 4 {
        a.square()
    }
    end_timer!(start);

    let start = start_timer!(|| "cpu512_square");
    for _ in 0..ROUND * 8 {
        a.square()
    }
    end_timer!(start);
}
