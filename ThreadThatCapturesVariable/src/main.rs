//Importing
use std::thread;
use std::time::Duration;
use std::io;
fn main() {
    //Taking Input
    println!("Please Enter How Many Numbers You  Want Add In A List? :");
    let mut count = String::new();
    io::stdin()
        .read_line(&mut count)
        .expect("Failed To Read Count!");
    let count: i32 = count.trim().parse()
        .expect("Failed To Convert Input Number Into i32!");
    
    let mut numbers: Vec<String> = Vec::new();
    for i in 0..count{
        println!("{}", format!("Please Enter {} Element To Enter In The List: ", i+1));
        //Taking Input
        let mut str_vec = String::new();
        io::stdin()
            .read_line(&mut str_vec)
            .expect("Failed To Read The String!");
        let str_vec: String = str_vec.trim()
            .parse()
            .expect("Failed To Convert The Input Into String!");

        //Storing Into Vector
        numbers.push(str_vec);
    }

    let a_thread = thread::spawn(move || {
        println!("Reading List! Please Wait A Little.");
        thread::sleep(Duration::from_secs(3));
        println!("\nThe Entered List Of String Is: ");
        for i in 0..numbers.len(){
            println!("{}", format!("The {} Element Is :\t{}", i + 1, numbers[i]));
        };
    });

    //Waiting For The thread
    a_thread.join().unwrap();
}