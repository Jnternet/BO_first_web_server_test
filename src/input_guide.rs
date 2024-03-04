use std::io::BufRead;
use std::time::SystemTime;
use walkdir::DirEntry;
use walkdir::Error;

pub fn 选择文件(v: &[(DirEntry, SystemTime)]) -> Option<DirEntry> {
    if v.is_empty() {
        return None;
    }
    println!("编号为0的是正确文件吗(回车/编号)");
    let mut num = 0;
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let parse_result = line.parse::<usize>();
        if let Err(e) = parse_result {
            eprintln!("输入错误, 请重新输入 原因: {e}");
            continue;
        }
        let parse_result = parse_result.unwrap();
        if parse_result > v.len() - 1 {
            println!("超出范围，请重新输入,回车使用最近的选择");
            continue;
        }
        num = parse_result;
        println!("选择的文件{}", v[num].0.file_name().to_str().unwrap());
        println!("重新选择？回车下一步");
    }
    println!("最终确定的文件{}", v[num].0.file_name().to_str().unwrap());
    Some(v[num].0.clone())
}
