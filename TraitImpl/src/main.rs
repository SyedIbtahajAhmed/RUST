//Importing HashMap
use std::collections::HashMap;

//Importing Display Trait
use std::fmt::Display;

//Making Struct
#[derive(Debug)]
struct Pair<T>{
    x: T,
    y: T,
}
//Implementing Structure Associate Function
//Can Call On Any Data Type
impl <T> Pair <T>{
    fn new(x: T, y: T) -> Pair <T>{
        Pair{
            x,
            y,
        }
    }
}

//Implementing Structure
//Can Call On Only Data Types Which Implement PartialOrder And Display
impl <T: PartialOrd + Display> Pair <T>{
    fn compare(&self){
        if self.x > self.y{
            println!("x Is Grater!");
        }
        else{
            println!("y Is Greater");
        }
    }
}

//Main Function
fn main() {
    //Integer And Float Implement PartialOrder Trait
    let int_type = Pair::new(
        6,
        8,
    );
    //String Data Type Does Not Implement Partial Order But Here We Are Calling Generic Data Type That is Why It Is Running Properly And
    //Comparing Based On The ASCII Codes
    let str_type = Pair::new(
        's',
        'b',
    );

    println!("{:?}", int_type);
    int_type.compare();

    //Works On ASCII Codes
    println!("{:?}", str_type);
    str_type.compare();

    //HashMap Does Not Implement PartialOrd That Is Why Code Below Will Give Error
    // let mut h1 = HashMap::new();
    // h1.insert("Blue", 10);
    // let mut h2 = HashMap::new();
    // h1.insert("Red", 20);

    // let h3 = HashMap::new(h1,h2);
    //h3.Compare();
}