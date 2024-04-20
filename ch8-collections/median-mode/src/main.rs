use std::collections::HashMap;

fn median(v: &Vec<i32>) -> i32 {
    let mut sorted_v = v.to_vec();
    sorted_v.sort();

    *sorted_v.get(sorted_v.len() / 2).unwrap_or(&0)
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut h = HashMap::new();

    // Construct a counter map
    for i in v {
        let occurrences = h.entry(i).or_insert(1);
        *occurrences += 1;
    }

    // Find the value with the corresponding max occurrences
    let mut max_occurrences = -1;
    let mut val = 0;
    for (i, occurrences) in h {
        if occurrences > max_occurrences {
            max_occurrences = occurrences;
            val = *i;
        }
    }

    val
}

fn main() {
    // Results should be the same
    let v1 = vec![1, 2, 2, 2, 2, 3, 3, 4, 4, 5, 5];
    let v2 = vec![5, 3, 2, 2, 5, 4, 3, 4, 2, 2, 1];

    for v in [v1, v2] {
        let median = median(&v);
        let mode = mode(&v);
        println!("Vector: {:?}", &v);
        println!("Median: {median}");
        println!("Mode: {mode}");
    }
}
