mod theme;
use dotenv::dotenv;
use std::error::Error;

use newsapi::{get_articles, Articles};

fn render_articles(articles: &Articles) {
    let theme = theme::default();
    // Markdown to cli
    theme.print_text("# Top headlines\n\n");
    for a in &articles.articles {
        theme.print_text(&format!("`{}`", a.title));
        theme.print_text(&format!("> *{}*", a.url));
        theme.print_text("---")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";
    let url = format!("{}{}", url, api_key);
    let articles = get_articles(&url)?;

    render_articles(&articles);
    Ok(())
}
