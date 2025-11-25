use std::{i32, isize};
use std::ops::Add;
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