// 从命令行参数接收一个文件路径，例如 input.txt。如果没有提供路径或文件无法打开，给出相应的错误提示并退出。
// 读取该文件的所有文本内容，统计文本中一共包含的字符数（不含换行符）与行数，并将结果写入 output.txt。
// 若 output.txt 文件已存在，可以选择直接覆盖或者追加，任选其一，但需要在程序里明确注释或说明处理方式。
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn get_file_name() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).ok_or("请提供输入文件路径")?;

    // 读取文件内容
    let content = fs::read_to_string(input_path)
        .map_err(|e| format!("无法读取文件 {}: {}", input_path, e))?;

    // 统计信息
    let stats = analyze_text(&content);

    let output_path = "output.txt";
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // 选择覆盖已有文件
        .open(output_path)?;

    // 写入统计结果
    writeln!(output_file, "文本分析结果：")?;
    writeln!(output_file, "行数: {}", stats.line_count)?;
    writeln!(output_file, "字符数（不含换行符）: {}", stats.char_count)?;

    println!("分析结束。");

    Ok(())
}

struct TextStats {
    line_count: usize,
    char_count: usize,
}

fn analyze_text(content: &str) -> TextStats {
    let line_count = content.lines().count();
    // 过滤换行符统计字符数
    let char_count = content.chars().filter(|&c| c != '\n').count();

    TextStats {
        line_count,
        char_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_text_empty_file() {
        let content = "";
        let stats = analyze_text(content);
        assert_eq!(stats.line_count, 0);
        assert_eq!(stats.char_count, 0);
    }

    #[test]
    fn test_analyze_text_single_line() {
        let content = "Hello, world!";
        let stats = analyze_text(content);
        assert_eq!(stats.line_count, 1);
        assert_eq!(stats.char_count, 13);
    }

    #[test]
    fn test_analyze_text_multiple_lines() {
        let content = "Hello\nRust\nWorld";
        let stats = analyze_text(content);
        assert_eq!(stats.line_count, 3);
        assert_eq!(stats.char_count, 14); // Excludes '\n'
    }
}
