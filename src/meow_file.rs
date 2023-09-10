use crate::config::Mode;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct MeowFile {
    pub filepath: String,
}

impl MeowFile {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }

    pub fn read_file(&self, mode: &Mode, line_number: &mut i32) -> Result<(), String> {
        let file = std::fs::File::open(&self.filepath);
        let reader = BufReader::new(file.unwrap());

        for line in reader.lines() {
            let unwrapped_line = line.unwrap();
            Self::process_line(unwrapped_line, line_number, mode);
        }

        Ok(())
    }

    fn is_numeric(s: &str) -> bool {
        s.chars().all(|c| c.is_numeric())
    }

    fn is_capitalised(s: &str) -> bool {
        s.chars().next().unwrap().is_uppercase()
    }

    pub fn process_line(line: String, line_number: &mut i32, mode: &Mode) {
        let unwrapped_line = line;
        let words = unwrapped_line.split(' ');
        let mut converted_words: Vec<String> = Vec::new();

        match mode {
            Mode::NumberLines => {
                converted_words.push(line_number.to_string());
                *line_number += 1;
            }
            Mode::NumberNonblankLines => {
                if unwrapped_line.is_empty() {
                    converted_words.push(" ".to_string());
                } else {
                    converted_words.push(line_number.to_string());
                    *line_number += 1;
                }
            }
            _ => {}
        }

        for word in words {
            converted_words.push(Self::process_word(word));
        }
        println!("{}", converted_words.join(" "));
    }

    fn process_word(word: &str) -> String {
        // skip if word is fully numeric (e.g. "2")
        if Self::is_numeric(word) {
            return word.to_string();
        }

        let mut replaced = String::new();

        match word.len() {
            0 => return "".to_string(),
            1..=4 => {
                for _ in 0..word.len() {
                    replaced.push('🐱');
                }
            }
            _ => {
                match Self::is_capitalised(word) {
                    true => replaced.push_str("Meo"),
                    false => replaced.push_str("meo"),
                }
                for _ in 0..word.len() - 3 {
                    replaced.push('w');
                }
            }
        }

        // add punctuation back
        // TODO: implement for multiple punctuation ending in the string
        if let Some(last_char) = word.chars().last() {
            if last_char.is_ascii_punctuation() {
                replaced.pop();
                replaced.push(last_char);
            }
        }
        replaced
    }
}

impl std::fmt::Display for MeowFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MeowFile: {}", self.filepath)
    }
}

#[cfg(test)]
mod tests {
    use super::MeowFile;
    use super::Mode;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_is_numeric() {
        assert!(MeowFile::is_numeric("123"));
        assert!(!MeowFile::is_numeric("abc"));
    }

    #[test]
    fn test_is_capitalised() {
        assert!(MeowFile::is_capitalised("Hello"));
        assert!(!MeowFile::is_capitalised("hello"));
    }

    #[test]
    fn test_read_file() {
        // Create a temporary test file
        let file_path = "test_file.txt";
        let mut file = File::create(file_path).unwrap();
        file.write_all(b"Hello world!").unwrap();

        let meow_file = MeowFile::new(file_path.to_string());
        let mut line_number = 0;
        let result = meow_file.read_file(&Mode::FileMode, &mut line_number);

        // Cleanup
        std::fs::remove_file(file_path).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_process_word() {
        let word = "Hello";
        let result = MeowFile::process_word(word);
        assert_eq!(result, "Meoww");
    }

    #[test]
    fn test_process_word_empty_string() {
        let word = "";
        let result = MeowFile::process_word(word);
        assert_eq!(result, "");
    }

    #[test]
    fn test_process_word_alphanumeric() {
        let word = "1234";
        let result = MeowFile::process_word(word);
        assert_eq!(result, "1234");
    }

    #[test]
    fn test_process_word_punctuation() {
        let word = "Hello,";
        let result = MeowFile::process_word(word);

        assert_eq!(result, "Meoww,");
    }

    #[test]
    fn test_process_word_less_than_4() {
        let word1 = "war";
        let result1 = MeowFile::process_word(word1);

        let word2 = "w";
        let result2 = MeowFile::process_word(word2);

        let word3 = "R";
        let result3 = MeowFile::process_word(word3);

        assert_eq!(result1, "🐱🐱🐱");
        assert_eq!(result2, "🐱");
        assert_eq!(result3, "🐱");
    }
}
