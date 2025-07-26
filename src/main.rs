use tokio;

mod analyze;
mod fetch;
mod save;

pub const PATH: &str = "./things.csv";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    save::deduplicate_csv(PATH)?;
    let urls: Vec<String> = vec![
        "https://rss.nytimes.com/services/xml/rss/nyt/HomePage.xml",
        "https://www.buzzfeed.com/index.xml",
        "https://www.theguardian.com/world/rss",
        "https://www.wired.com/feed/rss",
        "https://www.theguardian.com/us/rss",
        "https://www.theverge.com/rss/partner/subscriber-only-full-feed/rss.xml",
        "https://www.npr.org/rss/rss.php?id=1001",
        "https://mashable.com/feeds/rss/all",
        "https://moxie.foxnews.com/google-publisher/latest.xml",
        "https://nypost.com/feed/",
        "http://associated-press.s3-website-us-east-1.amazonaws.com/business.xml",
        "http://associated-press.s3-website-us-east-1.amazonaws.com/climate-and-environment.xml",
        "http://associated-press.s3-website-us-east-1.amazonaws.com/entertainment.xml",
        "http://associated-press.s3-website-us-east-1.amazonaws.com/health.xml",
        "http://associated-press.s3-website-us-east-1.amazonaws.com/lifestyle.xml",
        "http://associated-press.s3-website-us-east-1.amazonaws.com/oddities.xml",
        "http://associated-press.s3-website-us-east-1.amazonaws.com/politics.xml",
        "http://associated-press.s3-website-us-east-1.amazonaws.com/science.xml",
        "http://associated-press.s3-website-us-east-1.amazonaws.com/sports.xml",
        "http://associated-press.s3-website-us-east-1.amazonaws.com/technology.xml",
        "http://associated-press.s3-website-us-east-1.amazonaws.com/travel.xml",
        "http://associated-press.s3-website-us-east-1.amazonaws.com/us-news.xml",
        "http://associated-press.s3-website-us-east-1.amazonaws.com/world-news.xml",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let titles = fetch::fetch(urls).await?;

    println!("{:#?}", titles);
    println!("Total titles: {}", titles.len());

    let titles = analyze::analyze(titles).await?;

    dbg!(&titles);
    save::deduplicate_csv(PATH)?;

    Ok(())
}
