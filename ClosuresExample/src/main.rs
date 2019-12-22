fn main() {
    let sum_numbers = |num: i32 , num1: i32| -> i32{
        num + num1
    };

    println!("Using Closures: {}", sum_numbers(2,6));
    println!("Using Function: {}", Sum_func(23, 50));
}

fn Sum_func(x: i32, y: i32) -> i32{
    x+y
}