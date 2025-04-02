// 从命令行读取一个整数 n（若读取失败或没有输入则默认 n = 5）。
// 打印从 1 到 n 的所有整数，每行一个。
// 若该整数可以被 3 整除，则在数字后面附加输出 "Fizz"；若可以被 5 整除，则附加输出 "Buzz"；若同时满足可以被 3 和 5 整除的情况，则输出 "FizzBuzz"。
use std::io::{self, BufRead};

pub fn set_num_env_args() {
    let mut input = String::new();
    println!("Please input number: ");
    io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("Fail to get input.");

    num_addition_string(input);
}

pub fn num_addition_string(input: String) {
    let num: i32 = input.trim().parse().unwrap_or(5);

    for i in 1..=num {
        match (i % 3, i % 5) {
            (0, 0) => println!("{i} FizzBuzz"),
            (_, 0) => println!("{i} Buzz"),
            (0, _) => println!("{i} Fizz"),
            _ => println!("{i}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_args() {
        let input = String::from("");
        num_addition_string(input);
    }

    #[test]
    fn test_with_10() {
        let input = String::from("10");
        num_addition_string(input);
    }

    #[test]
    fn test_with_negative_number() {
        let input = String::from("abc");
        num_addition_string(input);
    }
}
