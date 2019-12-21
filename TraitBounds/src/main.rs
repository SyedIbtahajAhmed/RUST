use std::fmt::Display;
#[derive(Debug)]
struct Pair<T> {
    num1: T,
    num2: T,
}

impl <T> Pair <T>{
    fn asso_pair(x: T, y: T) -> Pair<T>{
        Pair{
            num1: x,
            num2: y,
        }
    }
}

impl <T: PartialOrd + Display> Pair<T>{
    fn greater(&self){
        if self.num1 > self.num2 {
            println!("{} Is Greater Than {}!", self.num1, self.num2);
        }else{
            println!("{} Is Greater Than {}!", self.num2, self.num1);
        }
    }
}


fn main() {
    let ins_1 = Pair::asso_pair(
        132,
        8902,
    );

    ins_1.greater();
}
