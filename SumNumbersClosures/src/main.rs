fn main() {
    //A Closure
    let return_sum = |num1, num2, num3|{
        num1 + num2 + num3
    };
    let num1 = 3;
    let num2 = 5; 
    let num3 = 10;
    println!("The Sum Of {}, {}, and {} Is : {}", num1, num2, num3, return_sum(num1, num2, num3));
}
