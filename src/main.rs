use chrono::{DateTime, Utc};
use regex::Regex;
use reqwest;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::string;

#[tokio::main]
async fn main() {
    let mut visited = vec![];
    let mut url: String = "https://stackoverflow.com/".to_string();
    let mut prev_url: String;
    let date: DateTime<Utc> = Utc::now();
    let log_file: String = format!("{}.log", date.format("%H-%M-%S %F"));
    let mut f = File::create(format!("./logs/{}", log_file)).expect("Unable to create file.");

    let url_regex = Regex::new(r#"<a\s+.+?[>]"#).unwrap();
    let href_regex = Regex::new(r#"href=.+?["].+?["]"#).unwrap();
    let https_regex = Regex::new(
        r#"(http|ftp|https)://([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&:/~+#-]*[\w@?^=%&/~+#-])"#,
    )
    .unwrap();
    let domain_regex = Regex::new(r#"^(?:https?://)?(?:[^@\n]+@)?(?:www\.)?([^:/n?]+)"#).unwrap();
    println!("Writing to {}", log_file);
    while (true) {
        visited.push(format!("{}", url));
        f.write_all(format!("{}\n", url).as_bytes())
            .expect("Unable to write data.");
        let body = reqwest::get(url.clone()).await.unwrap().text().await;
        let current_domain = domain_regex.captures(url.as_str()).unwrap();
        prev_url = format!("{}", url);
        for cap in url_regex.captures_iter(format!("{:?}", body).as_str()) {
            if cap.len() == 0 {
                continue;
            }
            let href_cap = href_regex.captures(&cap[0]);
            if href_cap.is_none() {
                continue;
            }
            let href_cap = href_cap.unwrap();
            let url_cap = https_regex.captures(&href_cap[0]);
            if url_cap.is_none() {
                continue;
            }
            let url_cap = url_cap.unwrap();
            let urll = format!("{}", &url_cap[0]);
            let domain_cap = domain_regex.captures(&url_cap[0]).unwrap();
            if &domain_cap[0] == &current_domain[0] {
                continue;
            }
            if visited.contains(&urll) {
                continue;
            }
            url = urll;
            break;
        }
        if prev_url == url {
            url = visited[visited.len() - 2].to_string();
        }
        println!("On {}", url);
    }
}
