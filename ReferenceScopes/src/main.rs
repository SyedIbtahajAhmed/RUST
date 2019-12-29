// Scope Of A Reference Starts where it is declared and ends at where it is called
fn main() {
    let mut string = String::from("Syed Ibtahaj");
    let ref1 = &string;     //Read Reference
    println!("{:p}", ref1);
    let ref2 = &mut string;      //Write Reference

    // println!("{} {}", ref1, ref2);      //This Is Wrong
    println!("{:p}", ref2);
}