use std::fs;

const FILE_PATH: &str = "input.txt";

fn main() {
    let (l1, l2) = read_file();
    let sum = count_naive(l1, l2);
    println!("total distance: {sum}");

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

fn count_naive(l1: Vec<i32>, l2: Vec<i32>) -> i32 {
    let mut l1 = l1.clone();
    let mut l2 = l2.clone();
    l1.sort();
    l2.sort();
    let sum: i32 = iter::zip(l1, l2)
        .map(|(n1, n2)| -> i32 { (n1 - n2).abs() })
        .sum();
    return sum;
}
