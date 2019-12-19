//Liftimes
fn lifetime<'a> (x: &'a str, mut y: &'a str) -> &'a str{
    {
        let x = "Ibtahaj";
        y = &x;
    }
    y
}
fn main() {
    let mut str1 = "ABCD";
    let mut str2 = "XYZ";
    let result = lifetime(str1, str2);
    println!("The String Is : {}", result);
}