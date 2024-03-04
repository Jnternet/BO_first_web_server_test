use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::DirEntry;

#[allow(deprecated)]
pub fn 展示文件及修改时间(v: &[(DirEntry, SystemTime)]) {
    v.iter().enumerate().for_each(|(x, (a, b))| {
        println!(
            "{x}: {}, \t最后修改日期: {}",
            a.file_name().to_str().unwrap(),
            DateTime::<Local>::from_utc(
                NaiveDateTime::from_timestamp(
                    b.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                    0,
                ),
                FixedOffset::east_opt(28800).unwrap(),
            )
        )
    });
}
