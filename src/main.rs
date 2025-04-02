mod question1;
mod question2;
mod question3;
mod question4;
mod question5;
mod question6;

fn main() {
    println!("Hello, world!");
    // 运行时获取number值
    // question1::set_num_env_args();
    // 运行时获取string值
    // question3::set_string_args();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(1 + 1, 2);
    }
}
