// 请从命令行读取一行字符串（例如 "apple banana pear banana apple banana"）。
// 使用空格进行拆分，统计每个单词出现的次数，并以从高到底的顺序输出。
// 如果出现次数相同，按单词本身的字典序从小到大排序输出。
use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn set_string_args() {
    println!("Please input a line string: ");
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let _ = count_words(input);
}

fn count_words(input: String) -> Vec<String> {
    let mut word_count = HashMap::new();
    for word in input.split_whitespace() {
        *word_count.entry(word.to_string()).or_insert(0) += 1;
    }

    let mut word_count_vec: Vec<_> = word_count.into_iter().collect();
    word_count_vec.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    let mut output: Vec<String> = vec![];

    for (word, count) in word_count_vec {
        println!("{word}: {count}");
        output.push(format!("{word}: {count}"));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let input = "";
        let output = count_words(input.to_string());
        assert!(output.is_empty());
    }

    #[test]
    fn test_multiple_words() {
        let input = "apple banana pear banana apple banana";
        let output = count_words(input.to_string());
        assert_eq!(output, vec!["pear: 1", "apple: 2", "banana: 3"]);
    }
}
