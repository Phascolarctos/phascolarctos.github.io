use gray_matter::{Matter, engine::YAML};
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
#[derive(Serialize)]
struct PostIndex {
    id: String,
    title: String,
    date: String,
    tags: Vec<String>,
    filename: String,
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
                let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                let content = fs::read_to_string(&path).expect("无法读取文件");
                println!("解析内容：{:?}", content);

                let result = matter.parse::<PostMetadata>(&content).ok();

                println!("解析结果：{:?}", result);
                if let Some(Some(mut meta)) = result.map(|r| r.data) {
                    if meta.id.is_empty() {
                        meta.id = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_string())
                            .unwrap_or_default();
                    }
                    posts.push(PostIndex {
                        id: meta.id,
                        title: meta.title,
                        date: meta.date,
                        tags: meta.tags,
                        filename: filename,
                    });
                }
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
