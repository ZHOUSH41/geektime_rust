use clap::Parser;
use reqwest::Url;
use anyhow::Result;

#[derive(Debug, Parser)]
#[clap(version = "1.0", author = "Me")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}
#[derive(Debug, Parser)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

struct Get {
    #[clap(parse(try_from_str=parse_url))]
    url: String,
}

struct Post {
    #[clap(parse(try_from_str=parse_url))]
    url: String,
    #[clap(parse(from_str=parse_kv_pair))]
    body: Vec<KvPair>,
}

#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}



fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into()) 
}

fn 

#[tokio::main]
async fn main() -> Result<()>{
    Ok(())
}

