use clap::Parser;
use reqwest;
use serde_json;
use serde_json::Value;
use async_recursion::async_recursion;
use chrono::prelude::*;
use std::fs::{self};
use std::io::prelude::*;
use std::io::LineWriter;

#[derive(Parser, Debug)]
#[clap(author = "myuan", version = "0.2", about = "华大基因云盘批量下载器", long_about = None)]
struct Args {
    /// shared url from BGI
    #[clap(short, long, value_parser)]
    url: String,

    /// export to
    #[clap(short, long, value_parser, default_value = "./download-from-bgi")]
    export_root: String,

    /// extraction code
    #[clap(short, long, value_parser, default_value = "")]
    code: String,

}

#[derive(Debug)]
struct DownloadArgs {
    path: String,
    row_id: String,
}

#[async_recursion]
async fn find_all_files(
    sess: &reqwest::Client, base_rows: &Vec<Value>, share_event_id: &str, code: &str, tab: usize
) -> Option<Vec<DownloadArgs>> {
    let mut res: Vec<DownloadArgs> = vec![];

    for row in base_rows {
        let path = row["path"].as_str().unwrap();
        println!("{}{}", "\t".repeat(tab), path);

        if row["type"] == "directory" {
            let rows_res = sess.post("https://pan.genomics.cn/ucdisk/api/2.0/share/link/rec/list/dir")
                .form(&(
                    ("pageNumber", 1),
                    ("pageSize", 500),
                    ("keyword", ""),
                    ("shareEventId", share_event_id),
                    ("nodeId", &row["id"]),
                    ("code", code),
                )).send().await.unwrap().json::<Value>().await.unwrap();

            res.append(&mut find_all_files(
                sess,
                rows_res["body"]["rows"].as_array().unwrap(),
                share_event_id,
                code, 
                tab + 1
            ).await?)
        } else {
            res.push(DownloadArgs{
                path: path.to_string(),
                row_id: row["id"].as_str().unwrap().to_string(),
            })
        }
    }
    Some(res)
}

fn check_os_and_aria2c() -> String {
    use std::env;

    let res = match env::consts::OS {
        "windows" => "./aria2c.exe",
        _ => "aria2c",
    }.to_string();

    let version = std::process::Command::new(res.as_str())
        .arg("--version")
        .output();
    match version {
        Ok(v) => {
            if !std::str::from_utf8(&v.stdout).unwrap().contains("Tatsuhiro Tsujikawa") {
                panic!("aria2 version not found");
            }
        }
        Err(e) => {
            panic!("aria2 is not installed {}", e);
        }
    }
    res
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let aria2c = check_os_and_aria2c();

    let args = Args::parse();
    let download_url: String = args.url;
    let export_root: String = args.export_root;
    let code: String = args.code;


    let sess = reqwest::Client::new();

    let url_meta = sess.post("https://pan.genomics.cn/ucdisk/api/2.0/share/link/shareLongUrl")
        .form(&vec![("shortUrl", &download_url)])
        .send().await?.json::<Value>().await?;

    let share_event_id = url_meta["body"]["bean"]["id"].as_str().unwrap();
    let share_from = url_meta["body"]["bean"]["senderName"].as_str().unwrap();
    println!("share from: {} {}", share_from, share_event_id);

    let base_dir_json = sess.post("https://pan.genomics.cn/ucdisk/s/api/2.0/share/link/info")
        .form(&(
            ("shareEventId", share_event_id),
            ("pageNumber", 1),
            ("pageSize", 500),
            ("code", &code),
        )).send().await?.json::<Value>().await?;

    let base_rows: &Vec<Value> = &base_dir_json["body"]["rows"].as_array().unwrap();
    let all_files: Vec<DownloadArgs> = find_all_files(&sess, base_rows, share_event_id, &code, 0).await.unwrap();

    let base_dir = std::format!("{}/{}", export_root, Local::now().format("%Y%m%d"));
    let download_list_file = fs::File::create("./download_list.txt").unwrap();
    let mut download_list_file = LineWriter::new(download_list_file);

    for file in all_files {
        download_list_file.write_all(format!("https://pan.genomics.cn/ucdisk/api/2.0/share/link/download?shareEventId={}&nodeId={}&code={}\n", share_event_id, file.row_id, code).as_bytes()).unwrap();
        download_list_file.write_all(format!("\tout={}\n", file.path).as_bytes()).unwrap();
    }
    println!("download to {}", base_dir);

    let mut pros = std::process::Command::new(aria2c)
        .arg("-i")
        .arg("./download_list.txt")
        .arg("-d")
        .arg(&base_dir)
        .arg("-c")
        .spawn()
        .expect("aria2c failed to start");

    println!("{}", pros.wait().expect("aria2c failed to finish"));

    Ok(())
}
