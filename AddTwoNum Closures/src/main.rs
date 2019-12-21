//Closures Add Two Numbers
fn main() {
    let add_num = |num1, num2|{
        num1 + num2
    };
    let num1 = 10;
    let num2 = 20;
    //Printing Function's Result
    println!("\nThe Addition Of {} And {} Is: {}", num1, num2, add_num(num1, num2));
}
