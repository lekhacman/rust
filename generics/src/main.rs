
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("ofcourse, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify(tweet)
}

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_2<T> (item: T)
where T: Summary,
{

}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
