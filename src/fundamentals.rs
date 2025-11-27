//use std::str::pattern::SearchStep;
use std::{i32, isize};
use std::ops::{Add, Mul};
use std::cmp::PartialOrd;

//Insertion sort
pub fn insertion_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let k = arr[i];
        let mut j = i as isize - 1;

        while j > -1 && arr[j as usize] > k {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }

        arr[(j + 1) as usize] = k;
    }
}

//Merge sort divide-and-conquer
pub fn merge<T: PartialOrd + Copy>(arr: &mut [T], mid: usize) {
    let mut temp: Vec<T> = Vec::with_capacity(arr.len());
    let (left, right) = arr.split_at(mid);

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            temp.push(left[i]);
            i += 1;
        } else {
            temp.push(right[j]);
            j += 1;
        }
    }

    temp.extend_from_slice(&left[i..]);
    temp.extend_from_slice(&right[j..]);
    
    arr.copy_from_slice(&temp);
}

pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let length = arr.len();
    if length <= 1 {
        return;
    }

    let mid = length / 2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);
    merge(arr, mid);
}

//Maximum subarray divide-and-conquer
pub trait NumLike: Copy + PartialOrd + Add<Output = Self> + Default {
    fn neg_inf() -> Self;
}

impl NumLike for i32 {
    fn neg_inf() -> Self {
        i32::MIN
    }
}

impl NumLike for i64 {
    fn neg_inf() -> Self {
        i64::MIN
    }
}

impl NumLike for isize {
    fn neg_inf() -> Self {
        isize::MIN
    }
}

fn find_max_crossing_subarray<T: NumLike>(arr: &[T], low: usize, mid: usize, high: usize) -> (usize, usize, T) {
    let mut left_sum = T::neg_inf();
    let mut sum = T::default();
    let mut max_left = 0;
    
    for i in (low..=mid).rev() {
        sum = sum + arr[i];

        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }

    let mut right_sum = T::neg_inf();
    let mut max_right = 0;
    sum = T::default();

    for j in (mid + 1)..=high {
        sum = sum + arr[j];

        if sum > right_sum {
            right_sum = sum;
            max_right = j;
        }
    }

    (max_left, max_right, left_sum + right_sum)
}

pub fn find_maximum_subarray<T: NumLike>(arr: &[T], low: usize, high: usize) -> (usize, usize, T) {
    if high == low {
        return (low, high, arr[low]);
    } else {
        let mid = (low + high) / 2;
        let (left_low, left_high, left_sum) = find_maximum_subarray(&arr, low, mid);
        let (right_low, right_high, right_sum) = find_maximum_subarray(&arr, mid + 1, high);
        let (cross_low, cross_high, cross_sum) = find_max_crossing_subarray(&arr, low, mid, high);

        if left_sum >= right_sum && left_sum >= cross_sum {
            return (left_low, left_high, left_sum);
        } else if right_sum >= left_sum && right_sum >= cross_sum {
            return (right_low, right_high, right_sum);
        } else {
            return (cross_low, cross_high, cross_sum);
        }
    }
}

//Matrix multiplication
type Matrix = Vec<Vec<f64>>;

fn zeroes(n: usize) -> Matrix {
    vec![vec![0.0; n]; n]
}

fn add(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let mut c = zeroes(n);

    for i in 0..n {
        for j in 0..n {
            c[i][j] = a[i][j] + b[i][j]
        }
    }
    c
}

fn slice(a: &Matrix, row: usize, col: usize, n: usize) -> Matrix {
    let mut m = zeroes(n);

    for i in 0..n {
        for j in 0..n {
            m[i][j] = a[row + i][col + j];
        }
    }
    m
}

fn place(dest: &mut Matrix, src: &Matrix, row: usize, col: usize) {
    let n = src.len();
    for i in 0..n {
        for j in 0..n {
            dest[row + i][col + j] = src[i][j]; 
        }
    }
}

//Brute force
pub fn square_matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let mut c = zeroes(n);

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] = c[i][j] + a[i][k] * b[k][j];
            }
        }
    }

    c
}

//Divide-and-conquer
pub fn square_matrix_multiply_recursive(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();

    if n == 1 {
        return vec![vec![a[0][0] * b[0][0]]];
    }

    let k = n / 2;

    let a11 = slice(a, 0, 0, k);
    let a12 = slice(a, 0, k, k);
    let a21 = slice(a, k, 0, k);
    let a22 = slice(a, k, k ,k);

    let b11 = slice(b, 0, 0, k);
    let b12 = slice(b, 0, k, k);
    let b21 = slice(b, k, 0, k);
    let b22 = slice(b, k, k ,k);

    let c11 = add(
        &square_matrix_multiply_recursive(&a11, &b11),
        &square_matrix_multiply_recursive(&a12, &b21)
    );

    let c12 = add(
        &square_matrix_multiply_recursive(&a11, &b12),
        &square_matrix_multiply_recursive(&a12, &b22)
    );

    let c21 = add(
        &square_matrix_multiply_recursive(&a21, &b11),
        &square_matrix_multiply_recursive(&a22, &b21)
    );

    let c22 = add(
        &square_matrix_multiply_recursive(&a21, &b12),
        &square_matrix_multiply_recursive(&a22, &b22)
    );

    let mut c = zeroes(n);

    place(&mut c, &c11, 0, 0);
    place(&mut c, &c12, 0, k);
    place(&mut c, &c21, k, 0);
    place(&mut c, &c22, k, k);

    c
}