fn main() {
    let return_two = |num1, num2|{
        println!("{} + {} = {}", num1, num2, num1+num2)
    };

    return_two(2,5);

    let func = |num1|{
        println!("{} + {} = {}", num1, 2, num1 + 2)
    };
    func(2);
}
