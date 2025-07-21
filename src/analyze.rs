use kalosm::language::*;
use rand::rng;
use rand::seq::SliceRandom;

use crate::save;

#[derive(Debug, Parse, Clone, Schema)]
struct Ranking {
    #[parse(range=0..=100)]
    pub clickbaitiness: i8,
}

pub async fn analyze(titles: Vec<String>) -> Result<Vec<(String, i8)>, Box<dyn std::error::Error>> {
    let model = Llama::new_chat().await.unwrap();
    let task: Task<Llama, ArcParser<Ranking>> =
        model.task("Evaluate the following news article title and assign a clickbaitiness score between 1 and 100. A score of 1 indicates that the title is very straightforward and lacks sensationalism, while a score of 100 signifies that the title is highly sensational, misleading, or designed to provoke strong emotional reactions.")
        .typed::<Ranking>();

    let mut result: Vec<(String, i8)> = vec![];
    let mut titles = titles;
    titles.shuffle(&mut rng());

    for title in titles {
        let ranking = analyze_title(&title, &task).await;
        if let Err(e) = ranking {
            eprintln!("Error analyzing title '{}': {}", title, e);
            continue;
        } else if let Ok(count) = ranking {
            dbg!(&title, &count.clickbaitiness);
            result.push((title, count.clickbaitiness));
        }
    }
    Ok(result)
}

async fn analyze_title(
    title: &String,
    task: &Task<Llama, ArcParser<Ranking>>,
) -> Result<Ranking, Box<dyn std::error::Error>> {
    let result = task(&title).await?;
    save::append_to_csv("./things.csv", (&title, result.clickbaitiness))?;
    Ok(result)
}
