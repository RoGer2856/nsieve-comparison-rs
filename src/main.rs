use std::fmt::Display;

use bitvec::prelude::*;
use num::{
    traits::{AsPrimitive, WrappingShl},
    CheckedAdd, Integer, NumCast,
};

fn nsieve_unsafe<
    T: Integer + CheckedAdd + WrappingShl + NumCast + AsPrimitive<usize> + Display + AsPrimitive<T>,
>(
    n: T,
) {
    let mut count = 0;
    let mut flags = bitvec![u32, LocalBits; 0; n.as_()];
    for i in num_iter::range(T::from(2).unwrap(), T::from(n).unwrap()) {
        if unsafe { !flags.get_unchecked(AsPrimitive::<usize>::as_(i)) } {
            count += 1;
            for j in num_iter::range_step(i.shl(1), n, i) {
                unsafe { flags.set_unchecked(j.as_(), true) };
            }
        }
    }
    println!("Primes up to {:8} {:8}", n, count);
}

fn main_run_with_num_unsafe<
    T: Integer + CheckedAdd + WrappingShl + NumCast + AsPrimitive<usize> + Display + AsPrimitive<T>,
>() {
    let n = std::env::args_os()
        .nth(1)
        .and_then(|s| s.into_string().ok())
        .and_then(|n| n.parse().ok())
        .unwrap_or(4);

    let init_value = T::from(10000).unwrap();
    for i in 0..3 {
        nsieve_unsafe::<T>(init_value.shl(n - i));
    }
}

fn nsieve_safe<
    T: Integer + CheckedAdd + WrappingShl + NumCast + AsPrimitive<usize> + Display + AsPrimitive<T>,
>(
    n: T,
) {
    let mut count = 0;
    let mut flags = bitvec![u32, LocalBits; 0; n.as_()];
    for i in num_iter::range(T::from(2).unwrap(), T::from(n).unwrap()) {
        if !flags[AsPrimitive::<usize>::as_(i)] {
            count += 1;
            for j in num_iter::range_step(i.shl(1), n, i) {
                flags.set(j.as_(), true);
            }
        }
    }
    println!("Primes up to {:8} {:8}", n, count);
}

fn main_run_with_num_safe<
    T: Integer + CheckedAdd + WrappingShl + NumCast + AsPrimitive<usize> + Display + AsPrimitive<T>,
>() {
    let n = std::env::args_os()
        .nth(1)
        .and_then(|s| s.into_string().ok())
        .and_then(|n| n.parse().ok())
        .unwrap_or(4);

    let init_value = T::from(10000).unwrap();
    for i in 0..3 {
        nsieve_safe::<T>(init_value.shl(n - i));
    }
}

fn nsieve_orig_unsafe(n: usize) {
    let mut count = 0;
    let mut flags = bitvec![u32, LocalBits; 0; n];
    for i in 2..n {
        if unsafe { !flags.get_unchecked(i) } {
            count += 1;
            for j in ((i << 1)..n).step_by(i) {
                unsafe { flags.set_unchecked(j, true) };
            }
        }
    }
    println!("Primes up to {:8} {:8}", n, count);
}

fn main_orig_unsafe() {
    let n = std::env::args_os()
        .nth(1)
        .and_then(|s| s.into_string().ok())
        .and_then(|n| n.parse().ok())
        .unwrap_or(4);

    for i in 0..3 {
        nsieve_orig_unsafe(10000 << (n - i));
    }
}

fn nsieve_orig_safe(n: usize) {
    let mut count = 0;
    let mut flags = bitvec![u32, LocalBits; 0; n];
    for i in 2..n {
        if !flags[i] {
            count += 1;
            for j in ((i << 1)..n).step_by(i) {
                flags.set(j, true);
            }
        }
    }
    println!("Primes up to {:8} {:8}", n, count);
}

fn main_orig_safe() {
    let n = std::env::args_os()
        .nth(1)
        .and_then(|s| s.into_string().ok())
        .and_then(|n| n.parse().ok())
        .unwrap_or(4);

    for i in 0..3 {
        nsieve_orig_safe(10000 << (n - i));
    }
}

fn main() {
    main_run_with_num_safe::<u32>();
    // main_run_with_num_safe::<u64>();

    // main_run_with_num_unsafe::<u32>();
    // main_run_with_num_unsafe::<u64>();

    // main_orig_safe();

    // main_orig_unsafe();
}
