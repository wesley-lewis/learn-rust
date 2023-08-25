pub fn run_traits() {
    // similar to interfaces in other languages
    let news_article = NewsArticle {
        headline: "Someone died".to_string(),
        location: "Bandra".to_string(),
        author: "Anonymous".to_string(),
        content: "Murder".to_string(),
    };

    let tweet = Tweet {
        username: String::from("wsly"),
        content: String::from("begginning rust"),
        reply: true,
        retweet: false,
    };

    let summary_tweet = tweet.summarize();
    let summary_news_article = news_article.summarize();

    println!("{}", summary_tweet);
    println!("{}", summary_news_article);
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}
