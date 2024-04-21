extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

use std::io;

const VOWELS: &[&str] = &["a", "e", "i", "o", "u"];

fn convert(s: &str) -> String {
    let i_words = s.unicode_word_indices().collect::<Vec<(usize, &str)>>();
    let graphemes = s.graphemes(true).collect::<Vec<&str>>();
    let mut conversion = String::new();
    let mut prev_end_idx: usize = 0;

    for &(word_index, word) in i_words.iter() {
        // Pig-latinize word
        let mut word_iter = word.graphemes(true);

        let first_letter = word_iter.next().expect("Cannot get first letter!");
        let rest_of_word: String = word_iter.collect();

        let pl_word;
        if VOWELS.contains(&first_letter) {
            pl_word = String::from(word) + "-" + "yay";
        } else {
            pl_word = rest_of_word + "-" + first_letter + "ay";
        }

        // Return whitespace and punctuation to new string
        for idx in prev_end_idx..word_index {
            conversion.push_str(&graphemes[idx])
        }

        // Add new word to new string
        conversion.push_str(&pl_word);

        prev_end_idx = word_index + word.len();
    }

    // Return whitespace and punctuation to new string
    for idx in prev_end_idx..s.len() {
        conversion.push_str(&graphemes[idx])
    }

    conversion
}

fn main() {
    'main: loop {
        println!("Loinquerisme Pig Latine?");
        println!("Enter some english to translate it to Pig Latin:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Yikes!");

        let conversion = convert(&input);

        println!("Your conversion:");
        println!("{conversion}");

        loop {
            println!("Go again? (Y/n)");
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).expect("Yikes!");
            let answer = answer.trim().to_lowercase();
            if answer == "y" {
                continue 'main;
            } else if answer == "n" {
                println!("Oink oink. Goodbye.");
                break 'main;
            } else {
                println!("Must be y/n");
            }
        }
    }
}
