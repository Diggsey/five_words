use std::collections::BTreeMap;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

const WORD_LIST: &str = include_str!("../wordlist.txt");

fn is_disjoint(a: [u8; 5], b: [u8; 5]) -> bool {
    for i in 0..5 {
        for j in 0..5 {
            if a[i] == b[j] {
                return false;
            }
        }
    }
    true
}

fn has_duplicate(a: [u8; 5]) -> bool {
    for i in 0..4 {
        if a[i] == a[i + 1] {
            return true;
        }
    }
    false
}

fn walk_inner(n: usize, j: usize, cur_set: &[u16], edges: &[Vec<bool>]) -> Vec<[u16; 5]> {
    let k = cur_set[j];
    let mask = &edges[k as usize];
    let new_set: Vec<_> = cur_set[j + 1..]
        .iter()
        .copied()
        .filter(|&i| mask[(i - k) as usize])
        .collect();
    let mut results = walk(n - 1, &new_set, edges);
    for result in &mut results {
        result[n] = k;
    }
    results
}

fn walk(n: usize, cur_set: &[u16], edges: &[Vec<bool>]) -> Vec<[u16; 5]> {
    if n == 0 {
        cur_set.iter().map(|w| [*w, 0, 0, 0, 0]).collect()
    } else if n <= cur_set.len() {
        (0..(cur_set.len() - n))
            .map(|j| walk_inner(n, j, cur_set, edges))
            .flatten()
            .collect()
    } else {
        Vec::new()
    }
}

fn walk_parallel(n: usize, cur_set: &[u16], edges: &[Vec<bool>]) -> Vec<[u16; 5]> {
    (0..(cur_set.len() - n))
        .into_par_iter()
        .map(|j| walk_inner(n, j, cur_set, edges))
        .flatten_iter()
        .collect()
}

fn main() {
    let mut set_to_word = BTreeMap::<_, Vec<_>>::new();
    for word in WORD_LIST.lines() {
        if let Ok(mut sorted_word) = TryInto::<[u8; 5]>::try_into(word.as_bytes()) {
            sorted_word.sort();
            if !has_duplicate(sorted_word) {
                set_to_word.entry(sorted_word).or_default().push(word);
            }
        }
    }

    let words: Vec<_> = set_to_word.keys().copied().collect();
    let edges: Vec<Vec<_>> = words
        .iter()
        .enumerate()
        .map(|(i, &a)| {
            words
                .iter()
                .enumerate()
                .skip(i)
                .map(|(j, &b)| i < j && is_disjoint(a, b))
                .collect()
        })
        .collect();

    let initial_set: Vec<_> = (0..words.len() as u16).collect();
    let results = walk_parallel(4, &initial_set, &edges);

    let mut total = 0;
    let mut output = String::new();
    for &result in &results {
        let mut combinations = 1;
        for (i, &w) in result.iter().rev().enumerate() {
            if i != 0 {
                output += " / ";
            }
            let actual_words = &set_to_word[&words[w as usize]];
            combinations *= actual_words.len();
            if actual_words.len() > 1 {
                output += "{";
            }
            output += &actual_words.join(", ");
            if actual_words.len() > 1 {
                output += "}";
            }
        }
        total += combinations;
        output += "\n";
    }
    println!("{}", output);
    println!(
        "Found {} solutions ({} with anagrams removed).",
        total,
        results.len()
    );
}
