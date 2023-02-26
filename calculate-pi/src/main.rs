use rug::{
    ops::{DivRounding, Pow},
    Assign, Float, Integer,
};
use std::{
    ops::{Div, Neg},
    time::Instant,
};
extern crate itertools;

use itertools::Itertools;

fn sub_strings(source: &str, sub_size: usize) -> Vec<String> {
    source
        .chars()
        .chunks(sub_size)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<_>>()
}

fn main() {
    let exponent = 8;
    println!("Calculating 10^{} digits of PI", exponent);
    let now = Instant::now();
    let mut digits = Integer::new();
    digits.assign(Integer::i_pow_u(10, exponent));
    let pi = chudnovsky(digits);
    let elapsed = now.elapsed();
    let pi = pi.to_string();
    let pi_split = sub_strings(&pi, 100);

    println!("{}", pi_split.join("\n"));
    println!("Elapsed: {:.2?}", elapsed);
}

fn basic_chudnovsky(digits: Integer) -> Integer {
    let a_val: Integer = Integer::from(13591409);
    let b_val: Integer = Integer::from(545140134);
    let c_val: f64 = 640320f64;
    let c3_24: f64 = c_val * c_val * c_val / 24.0;
    let d_val: Integer = Integer::from(426880);
    let e_val: Integer = Integer::from(10005);
    let mut k = 1f64;
    let mut a_k = digits.clone();
    let mut a_sum = digits.clone();
    let mut b_sum = Integer::new();
    loop {
        let a_k_mul =
            (-(6f64 * k - 5f64) * (2f64 * k - 1f64) * (6f64 * k - 1f64)) / (k * k * k * c3_24);
        let a_k_option = (&a_k * Float::with_val(1000, a_k_mul)).to_integer();
        a_k = match a_k_option {
            None => a_k,
            Some(a_k_option) => a_k_option,
        };
        // a_k = (a_k / (k * k * k * C3_24)).floor();
        a_sum += &a_k;
        b_sum += &a_k * Integer::from(k.trunc() as i32);
        k += 1f64;
        if a_k.to_i128() == Some(0) {
            break;
        }
    }
    return (d_val * (e_val * &digits).sqrt() * &digits) / ((a_val * a_sum) + (b_val * b_sum));
}

#[derive(Debug)]
struct PQT {
    p: Integer,
    q: Integer,
    t: Integer,
}

const DIGITS_PER_TERM: f64 = 14.181647462725477f64;

fn chudnovsky(digits: Integer) -> Integer {
    let digits_float = Float::with_val(1000, &digits);
    let n = digits_float.div(DIGITS_PER_TERM) + 1_u8;
    let n_int = n.to_integer();
    let b = match n_int {
        Some(n_int) => n_int,
        None => Integer::new(),
    };
    let digits_to_int = digits.to_u32();
    let digits_int = match digits_to_int {
        Some(d) => d,
        None => 0,
    };
    let pqt = bit_split(Integer::new(), b);
    let mut exp = Integer::new();
    exp.assign(Integer::i_pow_u(10, digits_int));
    let sqrt_c = Integer::from(10005_u32 * exp).sqrt();
    let pi = pqt.q * 426880_u32 * sqrt_c / pqt.t;
    return pi;
}

fn bit_split(a: Integer, b: Integer) -> PQT {
    let c3_24: u128 = (640320f64.pow(3) / 24.0).trunc() as u128;
    let mut pqt = PQT {
        p: Integer::new(),
        q: Integer::new(),
        t: Integer::new(),
    };
    let mut b_minus_a = Integer::new();
    b_minus_a.assign(&b - &a);
    if b_minus_a == 1_u8 {
        if a == 0_u8 {
            pqt.p.assign(1_u128);
            pqt.q.assign(1_u128);
        } else {
            let mut a_6 = Integer::new();
            a_6.assign(&a * 6_u8);
            let mut a_2 = Integer::new();
            a_2.assign(&a * 2_u8);
            pqt.p
                .assign((a_6.clone() - 5_u8) * (a_2 - 1_u8) * (a_6 - 1_u8));
            let mut a_pow_3 = Integer::new();
            a_pow_3.assign(a.clone().pow(3));
            pqt.q.assign(a_pow_3 * c3_24);
        }
        let mut a_mul = Integer::new();
        a_mul.assign(545140134_u32 * &a);
        pqt.t.assign(&pqt.p * (13591409_u32 + a_mul));
        if a.clone() & 1_u8 != 0_u128 {
            pqt.t = pqt.t.neg();
        }
    } else {
        let mut a_plus_b = Integer::new();
        a_plus_b.assign(&a + &b);
        let m = a_plus_b.div_trunc(2_u8);
        let am = bit_split(a, m.clone());
        let mb = bit_split(m, b);
        pqt.p.assign(&am.p * mb.p);
        pqt.q.assign(am.q * &mb.q);
        pqt.t.assign(mb.q * am.t + am.p * mb.t);
    }
    return pqt;
}
