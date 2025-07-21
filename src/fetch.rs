use reqwest;
use rss::Channel;

pub async fn fetch(urls: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok(get_gitles(&fetch_rss_feeds(urls).await?))
}

async fn fetch_rss_feeds(urls: Vec<String>) -> Result<Vec<Channel>, Box<dyn std::error::Error>> {
    let mut channels = Vec::new();
    for url in urls {
        let channel = fetch_rss_feed(&url).await;
        if let Err(e) = channel {
            eprintln!("Error fetching {}: {}", url, e);
            continue;
        } else if let Ok(channell) = channel {
            channels.push(channell);
        }
    }
    Ok(channels)
}

async fn fetch_rss_feed(url: &str) -> Result<Channel, Box<dyn std::error::Error>> {
    dbg!(url);

    let response = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&response[..])?;
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
