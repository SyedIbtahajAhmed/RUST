// Scope Of A Reference Starts where it is declared and ends at where it is called
fn main() {
    let mut string = String::from("Syed Ibtahaj");
    let ref1 = &string as *const String;     //Read Reference
    let ref2 = &mut string as *mut String;      //Write Reference

    // println!("{} {}", ref1, ref2);      //This Is Wrong

    // println!("{}, {}", ref1, ref2);      //We cannot use this without dereferencing it
    //so it will be used as
    unsafe{
        //As Ref 2 is mutable
        *ref2 = String::from("HELLO PAKISTAN");
        println!("{}, {}", *ref1, *ref2);
    }
}