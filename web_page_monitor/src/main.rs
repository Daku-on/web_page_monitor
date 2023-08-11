use reqwest::blocking::Client;
use scraper::{Html, Selector};

fn main() {
    let target_url = "https://cloud.google.com/vertex-ai/docs/tabular-data";
    let mut previous_contents = std::collections::HashMap::new();

    loop {
        match fetch_html(&target_url) {
            Ok(html_content) => {
                let document = Html::parse_document(&html_content);
                let links = extract_links(&document);

                for link in &links {
                    if let Some(prev_content) = previous_contents.get(link) {
                        if let Ok(new_content) = fetch_html(&link) {
                            if prev_content != &new_content {
                                println!("ページ '{}' が変更されました。", link);
                                println!("前回の内容:\n{}", prev_content);
                                println!("現在の内容:\n{}", new_content);
                                println!("{}", "=".repeat(50));

                            }
                        }
                    }
                }

                previous_contents.insert(target_url.to_string(), html_content);
                for link in &links {
                    let content = fetch_html(&link).unwrap_or_else(|_| String::new());
                    previous_contents.insert((&link).to_string(), content);
                }
            }
            Err(err) => eprintln!("エラー: {}", err),
        }

        std::thread::sleep(std::time::Duration::from_secs(60 * 500)); // 15分ごとにチェック
    }
}

fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = Client::new().get(url).send()?;
    Ok(response.text()?)
}

fn extract_links(document: &Html) -> Vec<String> {
    let mut links = Vec::new();
    let link_selector = Selector::parse("a[href]").unwrap();

    for link in document.select(&link_selector) {
        if let Some(href) = link.value().attr("href") {
            links.push(href.to_string());
        }
    }

    links
}
