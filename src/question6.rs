use clap::{Arg, Command};
use regex::Regex;
use std::fs;
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread;

fn main() {
    // 解析命令行参数
    let matches = Command::new("Rust 搜索工具")
        .about("在指定目录下搜索文本文件")
        .arg(
            Arg::new("keyword")
                .short('k')
                .long("keyword")
                .value_name("KEYWORD")
                .help("搜索关键词")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("dir")
                .short('d')
                .long("dir")
                .value_name("DIRECTORY")
                .help("目标目录")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("ignore_case")
                .short('i')
                .long("ignore-case")
                .help("忽略大小写")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("regex")
                .short('r')
                .long("regex")
                .help("使用正则匹配")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let keyword = matches.get_one::<String>("keyword").unwrap();
    let dir_path = matches.get_one::<String>("dir").unwrap();
    let ignore_case = matches.get_flag("ignore_case");
    let use_regex = matches.get_flag("regex");

    println!("搜索关键词: {}", keyword);
    println!("搜索目录: {}", dir_path);
    println!("忽略大小写: {}", ignore_case);
    println!("正则匹配: {}", use_regex);

    // 创建消息通道
    let (tx, rx) = mpsc::channel();

    // 获取所有 txt 文件
    let files = find_txt_files(dir_path);
    let mut handles = vec![];

    for file in files {
        let tx_clone = tx.clone();
        let keyword = keyword.to_string();
        let use_regex = use_regex;
        let ignore_case = ignore_case;

        let handle = thread::spawn(move || {
            if let Err(e) = search_in_file(&file, &keyword, ignore_case, use_regex, tx_clone) {
                eprintln!("文件 {} 读取失败: {}", file, e);
            }
        });

        handles.push(handle);
    }

    drop(tx); // 关闭发送端，避免 rx 阻塞

    // 结果收集
    let mut results = std::collections::HashMap::new();
    for received in rx {
        let (file, line_num, line) = received;
        results
            .entry(file)
            .or_insert_with(Vec::new)
            .push((line_num, line));
    }

    // 等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }

    // 输出搜索结果
    for (file, matches) in results {
        println!("\n文件: {}", file);
        for (line_num, line) in matches {
            println!("  行 {}: {}", line_num, line);
        }
    }
}

// 获取目录下的所有 `.txt` 文件
fn find_txt_files(dir: &str) -> Vec<String> {
    let mut files = vec![];
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map(|s| s == "txt").unwrap_or(false) {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }
    files
}

// 在文件中搜索关键字
fn search_in_file(
    file_path: &str,
    keyword: &str,
    ignore_case: bool,
    use_regex: bool,
    tx: mpsc::Sender<(String, usize, String)>,
) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let re = if use_regex {
        Some(Regex::new(keyword).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?)
    } else {
        None
    };

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let found = if let Some(re) = &re {
            re.is_match(&line)
        } else if ignore_case {
            line.to_lowercase().contains(&keyword.to_lowercase())
        } else {
            line.contains(keyword)
        };

        if found {
            tx.send((file_path.to_string(), line_num + 1, line))
                .unwrap();
        }
    }

    Ok(())
}
