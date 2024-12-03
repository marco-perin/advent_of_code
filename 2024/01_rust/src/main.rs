use std::fs;
use std::iter;

const FILE_PATH: &str = "input.txt";

fn main() {
    println!("##########");
    println!("# Part 1 #");
    println!("##########");
    println!();

    let (l1, l2) = read_file();
    let sum = count_distance(l1, l2);
    println!("Distance: {sum}");

    println!();
    println!("##########");
    println!("# Part 2 #");
    println!("##########");
    println!();

    let (l1, l2) = read_file();
    let sum = count_similarity(l1, l2);
    println!("Similarity: {sum}");
}

fn read_file() -> (Vec<i32>, Vec<i32>) {
    // Read file and append to list

    println!("Reading file...");

    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();

    let input = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file!");

    for line in input.lines() {
        let (n1, n2) = line.split_once("   ").unwrap();
        // println!("Line: `{n1}` `{n2}`"));
        l1.push(n1.parse().unwrap());
        l2.push(n2.parse().unwrap());
    }
    let l1_len = l1.len();

    println!("Parsed {l1_len} Lines");
    assert_eq!(l1.len(), l2.len());

    return (l1, l2);
}

fn count_distance(l1: Vec<i32>, l2: Vec<i32>) -> i32 {
    let mut l1 = l1.clone();
    let mut l2 = l2.clone();
    l1.sort();
    l2.sort();
    let sum: i32 = iter::zip(l1, l2)
        .map(|(n1, n2)| -> i32 { (n1 - n2).abs() })
        .sum();
    return sum;
}

fn count_similarity(l1: Vec<i32>, l2: Vec<i32>) -> i32 {
    let mut l1 = l1.clone();
    let mut l2 = l2.clone();
    l1.sort();
    l2.sort();
    let mut similarity = 0;

    let (mut i1, mut i2) = (0, 0);

    while i1 < l1.len() && i2 < l2.len() {
        let n = l1[i1];

        let (n1_times, n2_times);

        (n1_times, i1) = count_occurrence_in_sorted_list(n, &l1, Some(i1));
        (n2_times, i2) = count_occurrence_in_sorted_list(n, &l2, Some(i2));

        if n2_times > 0 {
            similarity += n * n2_times;
        }
    }
    return similarity;
}

fn count_occurrence_in_sorted_list(x: i32, l: &[i32], starting_idx: Option<usize>) -> (i32, usize) {
    let mut idx = starting_idx.unwrap_or(0);
    assert!(idx < l.len(), "Invalid index");

    let mut occurrs = 0;

    while idx < l.len() && x >= l[idx] {
        if x == l[idx] {
            occurrs += 1;
        }
        idx += 1;
    }
    return (occurrs, idx);
}
