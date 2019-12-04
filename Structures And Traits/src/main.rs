//STRUCT DEFINITION:
//The properties of an object is defined in an struct
//The Functionality Related to an object/Struct is returned in the implementation block

//ASSOCIATED FUNCTION
//A function within a implementation block of a certain object should return the instance of an object

//METHOD
//a function having a self parameter in the implementation block of a certain object is known as Method

#[derive(Debug)]
struct Tweet{
    uname: String,
    content: String,
}
#[derive(Debug)]
struct NewsArticle{
    author: String,
    content: String,
}

//Types Defining
pub trait Summary{
    fn summarize_author(&self) -> String;

    fn summarize(&self)->String;
}
//Types Implemented
impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("@{}Tweeted: {}", self.uname, self.content)
    }
}

impl Summary for NewsArticle{
    fn summarize_author(&self) -> String{
        format!("@{}Tweeted By The Author: {}", self.author, self.content)
    }
}

//For Default Implementation
impl Summary for NewsArticle{}

fn main(){
    let tweet1 = Tweet{
        uname: String::from("Ibtahaj"),
        content: String::from("Winter Is Coming!"),
    };

    let news1 = NewsArticle{
        author: String::from("John"),
        content: String::from("It Is Sunday!"),
    };

    println!("{}",tweet1.summarize());
    println!("{}",news1.summarize());
}