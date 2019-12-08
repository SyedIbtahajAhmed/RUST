fn main() {
    let int_list = vec![1,3,9];
    let char_list = vec!['a','z','e'];
    //For String Type Copy Trait Is Not Implemented
    //For String Clone Trait Is Used
    let str_list = vec![String::from("ibtahaj"),String::from("Ahmed")];

    println!("{}", largest_item(&int_list));
    println!("{}", largest_item(&char_list));
    println!("{}", largest_str(&str_list));
}

fn largest_item <T: PartialOrd + Copy> (item: &[T]) -> T {
    if type(T) == i32 || type(T) == f32{
        let mut largest = item[0];
        for &num in item.iter(){
            if num > largest{
                largest = num;
            }
        }
        largest
    }
    else if type(T) == String {
        let mut largest = &item[0];
        for &num in item.iter(){
            if num > largest{
                largest = num;
            }
        }
        largest
    }
}