use reqwest::Error;

pub async fn fetch_words(count: usize) -> Result<Vec<String>, Error> {
    let url = format!("https://random-word-api.vercel.app/api?words={}", count);
    let words: Vec<String> = reqwest::get(&url).await?.json().await?;
    Ok(words
        .into_iter()
        .filter(|w| w.len() > 3 && w.len() < 8)
        .collect())
}
