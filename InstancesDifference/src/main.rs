//Defining the Structure
#[derive(Debug)]
struct Student{
    name: String,
    id: i32,
    email: String,
}

//Making The Associative Function
impl Student{
    fn associative_std(nam: String, id: i32, em: String) -> Student{
        Student{
            name: nam,
            id: id,
            email: em,
        }
    }
}

fn main() {
    //Making An Instance Using Associative Function
    let std_1 = Student::associative_std(
        String::from("Syed Ibtahaj Ahmed Rizvi"),
        21,
        String::from("syd.ibthaj@gmail.com")
    );

    //Now Making An Instance Directly Using The Structure
    let std_2 = Student{
        name: String::from("Syed Saboor Ahmed Rizvi"),
        id: 19,
        email: "syed.saboor@gmail.com".to_string(),
    };

    //Printing On The Console
    println!("\nThe Instance Made with Associative Function:\n{:#?}", std_1);

    println!("\nThe Instance Made Directly:\n{:#?}", std_2);
}
