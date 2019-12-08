#[derive(Debug)]
struct Tweet{
    username: String,
    content: String,
}

#[derive(Debug)]
struct NewsArticle{
    author: String,
    content: String,
}

pub trait Summary{
    fn summarize_aut(&self) -> String;

    fn summarize(&self) -> String{
        format!("This Is from Default Implementation Of {}", self.summarize_aut())
    }
}

impl Summary for Tweet{
    fn summarize_aut(&self) -> String{
        format!("@{}", self.username)
    }
}

impl Summary for NewsArticle{
    fn summarize_aut(&self) -> String{
        format!("@{}", self.author)
    }
}

//Instance Maker Function
fn summarizeable() -> impl Summary{
    Tweet{
        username: String::from("Syed"),
        content: "BBBB".to_string(),
    }
}

fn main(){
    let tweet_1 = summarizeable();

    println!("{}",tweet_1.summarize());
}