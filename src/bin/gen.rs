use gray_matter::{Matter, engine::YAML};
use pulldown_cmark::{Options, Parser, html};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct PostMetadata {
    title: String,
    date: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    id: String,
}
#[derive(Serialize, Debug)]
struct PostIndex {
    id: String,
    title: String,
    date: String,
    tags: Vec<String>,
    filename: String,
    content: String,
}

const ARTICLES_DIR: &str = "./public/articles";
fn main() {
    let mut posts = Vec::new();

    let matter = Matter::<YAML>::new();

    if let Ok(entries) = fs::read_dir(ARTICLES_DIR) {
        for entry in entries.flatten() {
            let path = entry.path();
            let ext = path.extension().and_then(|s| s.to_str());
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                let content = fs::read_to_string(&path).expect("无法读取文件");
                println!("解析内容：{:?}", content);

                if let Ok(parsed) = matter.parse::<PostMetadata>(&content) {
                    let meta = parsed.data.unwrap();
                    let body = parsed.content;
                    let content_str=render(&body);
                    posts.push(PostIndex {
                        id: meta.id,
                        title: meta.title,
                        date: meta.date,
                        tags: meta.tags,
                        filename: file_name,
                        content: content_str,
                    });
                }
                println!("解析结果：{:?}", posts);
            }
        }
    }

    // 按日期倒序排列（新的在前）
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    let json = serde_json::to_string_pretty(&posts).unwrap();
    fs::create_dir_all("./public").ok();
    fs::write("./public/posts.json", json).expect("写入索引失败");

    println!("成功解析 {} 篇文章并生成索引！", posts.len());
}

pub fn render(markdown_str: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parse = Parser::new_ext(markdown_str, options);

    let mut html_str = String::new();

    html::push_html(&mut html_str, parse);

    html_str
}
