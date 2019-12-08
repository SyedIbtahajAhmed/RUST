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
//Trait Boundary Syntax
fn implsumm <T: Summary, U: Summary>(t: T, u: U, v: T){
    println!("{}\n{}\n{}", t.summarize(), u.summarize(), v.summarize())
}


fn main(){
    let tweet_1 = Tweet{
        username: "Syed Ibtahaj Ahmed".to_string(),
        content: "Its Sunday!".to_string(),
    };

    let news_1 = NewsArticle{
        author: "Mohammad Bial".to_string(),
        content: "Its Monday!".to_string(),
    };
    let tweet_2 = Tweet{
        username: "Mohammad Wasey".to_string(),
        content: "Its Friday!".to_string(),
    };

    implsumm(tweet_1, news_1, tweet_2);
}