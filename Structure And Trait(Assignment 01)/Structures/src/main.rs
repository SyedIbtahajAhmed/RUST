//USER DEFINED DATA TYPE(STRUCTURE):
#[derive(Debug)]
struct NewsArticle{
    author: String,
    content: String,
}

//USER DEFINED DATA TYPE(STRUCTURE):
#[derive(Debug)]
struct Tweet{
    username: String,
    content: String,
}

pub trait Summary{
    fn summarize(&self) -> String;
}

//Implementing Trait On NewsArticle Structure
impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("\nThe Name Of The Author Is: {}\nThe Content Is:\n{}\nThe Summary Is:\nThis Is Called From The NEWSARTICLE Trait", self.author, self.content)
    }
}

//Implementing Trait On Tweet Structure
impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("\nThe Name Of The Author Is: {}\nThe Content Is:\n{}\nThe Summary Is:\nThis Is Called From The NEWSARTICLE Trait", self.username, self.content)
    }
}

fn main() {
    //MAKING INSTANCE OF NEWSARTICLE STRUCTURE
    let news_1 = NewsArticle{
        author: "Syed Ibtahaj Ahmed".to_string(),
        content: "The Content Of This News Article Is Based On Todays Cricket Match!".to_string(),
    };

    //MAKING INSTANCE OF TWEET STRUCTURE
    let tweet_1 = Tweet{
        username: "Syed Shahid Ahmed".to_string(),
        content: "The Content Of This Tweet Is Based On Todays Party!".to_string(),
    };

    //CALLING SUMMARIZE METHOD FOR NEWS_1
    println!("{}", news_1.summarize());

    //CALLING SUMMARIZE METHOD FOR TWEET_1
    println!("{}", tweet_1.summarize());
}
