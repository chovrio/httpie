use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::Url;
// 定义 HTTPie的CLI的主入口，它包含若干个子命令
// 下面的///的注释是文档 clap 会将其作为 CLI 的帮助

// A native httpie implementation with Rust，can you imagine how easy it is?
#[derive(Debug, Parser)]
#[clap(version = "1.0", author = "chovrio")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommond,
}
// 子命令分别对应不同的HTTP请求 目前只支持 get / post
#[derive(Parser, Debug)]
enum SubCommond {
    Get(Get),
    Post(Post),
}
/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP请求的URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}
#[derive(Parser, Debug)]
struct Post {
    /// HTTP请求的URL
    url: String,
    /// HTTP请求的body
    body: Vec<String>,
}
fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}

fn parse_url(s: &str) -> Result<String> {
    // 这里我们仅仅检查一下 URL 是否合法
    let _url: Url = s.parse()?;
    Ok(s.into())
}
