// Module 9: Generics and Traits
// Task 4: Associated Types vs Generic Parameters
//
// Associated types let each implementation fix one concrete output type.
// This keeps APIs clear because callers do not need to specify generic
// type parameters at every usage site.
pub trait Summarize {
    type Output;
    fn summarize(&self) -> Self::Output;
}

pub struct Numbers {
    pub data: Vec<i32>,
}

pub struct Sentence {
    pub words: Vec<String>,
}

impl Summarize for Numbers {
    type Output = i32;

    fn summarize(&self) -> Self::Output {
        // Sum every integer in the vector.
        let mut total = 0;
        for value in &self.data {
            total += *value;
        }
        total
    }
}

impl Summarize for Sentence {
    type Output = String;

    fn summarize(&self) -> Self::Output {
        // Join words with single spaces.
        // Manual join keeps this compatible across Rust versions.
        let mut result = String::new();
        for (index, word) in self.words.iter().enumerate() {
            if index > 0 {
                result.push(' ');
            }
            result.push_str(word);
        }
        result
    }
}

pub fn run() {
    let nums = Numbers {
        data: vec![1, 2, 3, 4],
    };
    let sentence = Sentence {
        words: vec!["Rust".to_string(), "is".to_string(), "fun".to_string()],
    };

    println!("Numbers summarize = {}", nums.summarize());
    println!("Sentence summarize = {}", sentence.summarize());
}

#[cfg(test)]
mod tests {
    use super::{Numbers, Sentence, Summarize};

    #[test]
    fn summarizes_numbers_and_sentences() {
        let nums = Numbers {
            data: vec![5, 10, 15],
        };
        let sentence = Sentence {
            words: vec!["hello".to_string(), "rust".to_string()],
        };

        assert_eq!(nums.summarize(), 30);
        assert_eq!(sentence.summarize(), "hello rust");
    }
}
