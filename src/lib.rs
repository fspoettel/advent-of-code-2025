pub mod template;

// Use this file to add helper functions and additional modules.

pub fn is_even(val: usize) -> bool {
    val.is_multiple_of(2)
}

pub fn count_digits(n: usize) -> usize {
    (n.checked_ilog10().unwrap_or(0) + 1) as usize
}
