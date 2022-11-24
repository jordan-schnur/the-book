use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    // Pig latin generator.
    // First consonant of each word is moved to the end.
    // Words that start with a vowel have "yay" added to the end. ("Apple becomes "appleyay")
    // validator: https://lingojam.com/PigLatinTranslator

    let str = String::from("Relaxing in basins at the end of inlets terminates the endless tests from the box");
    let mut latinstr = String::new();

    for word in str.split_whitespace() {
        let first_letter = word.chars().next().unwrap();

        match first_letter {
            'a' | 'e' | 'i' | 'o' | 'u' => latinstr.push_str(format!("{}{} ", word, "yay").as_str()),
            _ => {
                let mut before_vowel = String::new();
                let mut after_vowel = String::new();
                let mut found_vowel = false;

                for (i, char) in word.chars().enumerate() {
                    if found_vowel {
                        after_vowel.push(char);
                        continue;
                    }

                    match char {
                        'a' | 'e' | 'i' | 'o' | 'u' => {
                            found_vowel = true;
                            after_vowel.push(char);
                        }
                        _ => before_vowel.push(char),
                    }
                }

                latinstr.push_str(format!("{}{}ay ", after_vowel, before_vowel).as_str());
            }
        }
    }

    println!("Final pig latin: {}", latinstr);
}
