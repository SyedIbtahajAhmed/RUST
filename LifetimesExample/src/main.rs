//Lifetimes Example
fn main() {
    println!("The Sum Is: {}", add_two_numbers(&mut 2, &3));
}

//Simple Addition Function:
// fn add_two_numbers (x: i32, y: i32) -> i32{
//     x + y;
// }

//Same As Above Funcions But Using Lfetimes
fn add_two_numbers <'a, 'b> (x: &'a mut i32, y: &'b i32) -> &'a i32{
    *x = *x + y;
    x
}