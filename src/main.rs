use clap::Parser;
use futures::stream::{self, StreamExt};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL to download from. Supports the following patterns:
    /// [start:end]: where start and end are an integer range. For each number within the range, the pattern will be replaced by the number and download will occur.
    /// [1:10]: 1,2,3,4,5,6,7,9,10
    /// [01:10]: 01,02,03,04,05,06,07,08,09,10
    #[clap(verbatim_doc_comment)]
    url: String,
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let patterns = parser::parse_patterns(&args.url);

    // TODO figure out what to do with multiple patterns
    if let Some(pattern) = patterns.get(0) {
        let request_client = reqwest::Client::new();
        let futures: Vec<_> = (pattern.start_int..=pattern.end_int)
            .map(|x| {
                let mut download_url = pattern.url.clone();
                download_url.replace_range(
                    pattern.start_index..pattern.end_index,
                    &format!("{:0pad$}", x, pad = pattern.pad),
                );
                download_content(download_url, &request_client)
            })
            .collect();
        let mut buffered_futures = stream::iter(futures).buffer_unordered(100);
        while let Some(res) = buffered_futures.next().await {
            println!("{:?}", res);
        }
    }
}

async fn download_content(
    url: String,
    client: &reqwest::Client,
) -> Result<String, Box<dyn std::error::Error>> {
    let file_name = &url.split('/').last().unwrap();
    let res = client.get(&url).send().await?;
    let mut file = File::create(file_name).await?;
    file.write_all(&res.bytes().await?).await?;
    Ok(format!("Wrote {}", file_name))
}
