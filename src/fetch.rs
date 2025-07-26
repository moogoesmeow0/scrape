use futures::future::join_all;
use reqwest;
use rss::Channel;

pub async fn fetch(urls: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok(get_gitles(&fetch_rss_feeds(urls).await?))
}

async fn fetch_rss_feeds(urls: Vec<String>) -> Result<Vec<Channel>, Box<dyn std::error::Error>> {
    // let mut channels = Vec::new();
    // for url in urls {
    //     let channel = fetch_rss_feed(&url).await;
    //
    //     if let Err(e) = channel {
    //         eprintln!("Error fetching {}: {}", url, e);
    //         continue;
    //     } else if let Ok(channell) = channel {
    //         channels.push(channell);
    //     }
    // }

    let futures = urls.iter().map(|url| fetch_rss_feed(url));
    let results: Vec<Result<Channel, Box<dyn std::error::Error>>> = join_all(futures).await;

    let channels: Vec<Channel> = results
        .into_iter()
        .zip(urls.iter())
        .filter_map(|(result, url)| match result {
            Ok(channel) => Some(channel),
            Err(e) => {
                eprintln!("Error fetching {}: {}", url, e);
                None
            }
        })
        .collect();

    Ok(channels)
}

async fn fetch_rss_feed(url: &str) -> Result<Channel, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&response[..])?;

    dbg!(url);
    Ok(channel)
}

fn get_gitle(channel: &Channel) -> Vec<String> {
    dbg!(channel.title());

    channel
        .items()
        .iter()
        .filter_map(|item| item.title().map(|t| t.to_string()))
        .collect()
}

fn get_gitles(channels: &Vec<Channel>) -> Vec<String> {
    channels
        .iter()
        .flat_map(|channel| get_gitle(channel))
        .collect()
}
