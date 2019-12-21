use std::io;
//Closures Prime Number Check
fn main() {
    let prime = |num| -> bool {
        let mut counting = 0;
        let mut count = num;
        let mut result = true;
        while count >= 2{
            count = count - 1;
            if count != 1{
                if num % count != 0 {
                    counting += 1;
                }
            }else{
                if counting == num - 2{
                    result = true;
                }else{
                    result = false;
                }
            }
        }
        result
    };

    //Taking User Input
    let mut num = String::new();
    print!("Please Enter Any Number:\n");
    io::stdin().read_line(&mut num).expect("Failed To Read Input!");
    let num: i64 = num.trim().parse().expect("Failed To Read Number");
    //Printing Result:
    println!("\nThe Number Entered Was: {}", num);
    println!("\nThe Result From Prime Number Checking Function Is: {}", prime(num));
}
