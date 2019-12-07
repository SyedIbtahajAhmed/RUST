//DEFINING USER DEFINED DATA TYPES STRUCTURES
#[derive(Debug)]
struct Book{
    author: String,
    name: String, 
}

//Defining Associative Function Of Book Structure
impl Book{
    fn associative_book(aut: String, nam: String) -> Book{
        Book{
            author: aut,
            name: nam,
        }
    }
}

//Defining Trait
pub trait BookInformation{
    fn info(&self) -> String;
}

//Impementing BookInformation Trait On Book Structure
impl BookInformation for Book{
    fn info(&self) -> String {
        format!("\nThe Name Of The Author  Is: {}\nThe Name Of The Book Is: {}", self.author, self.name)
    }
}
fn main() {
    //Making Instance Through And Associaive Function
    let book_1 = Book::associative_book(
        String::from("Syed Ibtahaj Ahmed Rizvi"), 
        "Linear Algebra And Differential Equations".to_string()
    );

    //Printing On The Console
    println!("{}", book_1.info());
}