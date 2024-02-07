
// 1. Implement a function that checks whether a given string is a palindrome or not.

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn main() {
    let s = "racecar";
    println!("Is '{}' a palindrome? {}", s, is_palindrome(s));
}



// 2. Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.


fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

fn main() {
    let arr = [1, 2, 3, 4, 4, 5, 6, 7];
    let target = 4;
    println!("First occurrence of {} is at index {:?}", target, first_occurrence(&arr, target));
}


// 3. Given a string of words, implement a function that returns the shortest word in the string.


fn shortest_word(s: &str) -> &str {
    s.split_whitespace().min_by_key(|word| word.len()).unwrap_or("")
}

fn main() {
    let s = "This is a test string";
    println!("Shortest word: {}", shortest_word(s));
}


// 4. Implement a function that checks whether a given number is prime or not.


fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let n = 17;
    println!("Is {} prime? {}", n, is_prime(n));
}


// 5. Given a sorted array of integers, implement a function that returns the median of the array.


fn median(arr: &[i32]) -> f64 {
    let mid = arr.len() / 2;
    if arr.len() % 2 == 0 {
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[mid] as f64
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("Median: {}", median(&arr));
}


// 6. Implement a function that finds the longest common prefix of a given set of strings.


fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let first = &strs[0];
    for (i, c) in first.chars().enumerate() {
        if strs.iter().any(|s| s.chars().nth(i) != Some(c)) {
            return first[..i].to_string();
        }
    }
    first.to_string()
}

fn main() {
    let strings = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("Longest common prefix: {}", longest_common_prefix(strings));
}


// 7. Implement a function that returns the kth smallest element in a given array.


fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted = arr.to_vec();
    sorted.sort();
    sorted.get(k - 1).cloned()
}

fn main() {
    let arr = [7, 10, 4, 3, 20, 15];
    let k = 3;
    println!("{}th smallest element: {:?}", k, kth_smallest(&arr, k));
}


8. Given a binary tree, implement a function that returns the maximum depth of the tree.

use std::cmp;

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            cmp::max(left_depth, right_depth) + 1
        }
        None => 0,
    }
}

fn main() {
    let mut root = TreeNode::new(1);
    root.left = Some(Box::new(TreeNode::new(2)));
    root.right = Some(Box::new(TreeNode::new(3)));
    root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
    root.left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));

    println!("Maximum depth of the tree: {}", max_depth(Some(Box::new(root))));
}


// 9. Reverse a string in Rust.


fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let s = "Hello, world!";
    println!("Reversed string: {}", reverse_string(s));
}


// 10. Check if a number is prime in Rust.


fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let n = 17;
    println!("Is {} prime? {}", n, is_prime(n));
}


// 11. Merge two sorted arrays in Rust.


fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);
    merged
}

fn main() {
    let arr1 = [1, 3, 5, 7];
    let arr2 = [2, 4, 6, 8];
    println!("Merged array: {:?}", merge_sorted_arrays(&arr1, &arr2));
}


// 12. Find the maximum subarray sum in Rust.


fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_ending_here = 0;
    let mut max_so_far = i32::MIN;

    for &x in arr {
        max_ending_here = max_ending_here.max(0) + x;
        max_so_far = max_so_far.max(max_ending_here);
    }
   

 max_so_far
}

fn main() {
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum subarray sum: {}", max_subarray_sum(&arr));
}
