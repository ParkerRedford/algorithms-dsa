pub mod fundamentals;
use fundamentals::*;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion_sort_test() {
        let mut arr = [8, 2, 5, 3, 4, 7, 6, 1];
        let mut arr1 = [2, 4, 5, 7, 1, 2, 5, 6];

        insertion_sort(&mut arr);
        insertion_sort(&mut arr1);

        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(arr1, [1, 2, 2, 4, 5, 5, 6, 7]);
    }

    #[test]
    fn merge_sort_test() {
        let mut arr =  [8, 2, 5, 3, 4, 7, 6, 1];
        let mut arr1 = [2, 4, 5, 7, 1, 2, 5, 6];
        
        merge_sort(&mut arr);
        merge_sort(&mut arr1);

        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(arr1, [1, 2, 2, 4, 5, 5, 6, 7]);
    }

    #[test]
    fn maximum_subarray_test() {
        let arr = [13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7];
        let subarray = find_maximum_subarray(&arr, 0, arr.len() - 1);

        assert_eq!(subarray, (7, 10, 43));
    }

    #[test]
    fn square_matrix_multiply_test() {
        let a = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0]
        ];

        let b = vec![
            vec![5.0, 6.0],
            vec![7.0, 8.0]
        ];

        //Brute force
        let c = square_matrix_multiply(&a, &b);

        //Divide-and-conquer
        let d = square_matrix_multiply_recursive(&a, &b);

        assert_eq!(c, [[19.0, 22.0], [43.0, 50.0]]);
        assert_eq!(d, [[19.0, 22.0], [43.0, 50.0]]);
    }
}