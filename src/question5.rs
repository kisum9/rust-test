// 使用多线程并行计算某个函数的值或模拟并发任务。
// 需要创建 3 个线程同时进行下载，并在下载完成后将结果（例如“URL + 下载完成”）
// 通过消息通道（std::sync::mpsc）发送回主线程。主线程依次接收并打印结果。

use std::{sync::mpsc, thread, time::Duration};

fn stimate_thread_download() -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    let urls = vec![
        "http://xxx.com/file1",
        "http://xxx.com/file2",
        "http://xxx.com/file3",
    ];

    for url in urls {
        let tx = tx.clone();
        thread::spawn(move || {
            println!("Start download..");
            thread::sleep(Duration::from_secs(2));
            tx.send(format!("url {url} Download Finish")).unwrap();
        });
    }

    drop(tx);
    let mut output: Vec<String> = vec![];

    for received in rx {
        println!("{}", received);
        output.push(format!("{}", received));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_results() {
        let results = stimate_thread_download();
        let expected_urls = vec![
            "url http://xxx.com/file1 Download Finish",
            "url http://xxx.com/file2 Download Finish",
            "url http://xxx.com/file3 Download Finish",
        ];

        // Ensure all expected URLs are in the results
        for expected in &expected_urls {
            assert!(results.contains(&(expected.to_string())));
        }

        // Ensure the number of results matches the number of URLs
        assert_eq!(results.len(), expected_urls.len());
    }
}
