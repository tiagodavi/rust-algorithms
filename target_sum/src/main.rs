/// Function: target_sum
///
/// Function to find whether or not two items from the list sum up to the target.
///
/// # Args (values: Vec<isize>, target: isize)
///
/// # Returns Vec<isize>
///
/// # Example
///
/// ```
/// let result : Vec<isize>  = target_sum([1,2,3,4], 5)
/// // [2, 3]
/// ```
// Time Complexity O(n^2)
// Space complexity: O(1)
// fn target_sum(values: Vec<isize>, target: isize) -> Vec<isize> {
//     let mut result: Vec<isize> = vec![];

//     'outer: for i in 0..values.len() - 1 {
//         for j in 1..values.len() {
//             if values[i] + values[j] == target {
//                 result.push(values[i]);
//                 result.push(values[j]);
//                 break 'outer;
//             }
//         }
//     }

//     result
// }

//// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

fn target_sum(values: Vec<isize>, target: isize) -> Vec<isize> {
    let mut temp: HashMap<isize, bool> = HashMap::new();
    let mut result: Vec<isize> = Vec::new();

    for val in values {
        let item = (target - val).abs();

        if temp.contains_key(&item) {
            result.push(item);
            result.push(val);
            break;
        } else {
            temp.insert(val, true);
        }
    }

    result
}

//// Time complexity: O(n log n)
// Space complexity: O(1)
// fn target_sum(mut values: Vec<isize>, target: isize) -> Vec<isize> {
//     let mut result: Vec<isize> = Vec::new();
//     values.sort();

//     let mut start: usize = 0;
//     let mut end = values.len() - 1;

//     while start < end {
//         let sum = values[start] + values[end];

//         if sum == target {
//             result.push(values[start]);
//             result.push(values[end]);
//             break;
//         } else if sum < target {
//             start += 1;
//         } else if sum > target {
//             end -= 1;
//         }
//     }

//     result
// }

fn main() {
    println!("Result is {:?}", target_sum(vec![1, 2, 6, 4], 5));
}

#[cfg(test)]
mod test {
    use super::target_sum;

    #[test]
    fn target_sum_returns_1_and_4() {
        assert_eq!(target_sum(vec![1, 7, 0, 4, 8], 5), vec![1, 4]);
    }

    #[test]
    fn target_sum_returns_8_and_1() {
        assert_eq!(target_sum(vec![1, 7, 9, 4, 8], 9), vec![1, 8]);
    }

    #[test]
    fn target_sum_returns_empty_vector() {
        assert_eq!(target_sum(vec![1, 7, 9, 4, 8], 20), Vec::new());
    }
}
