fn main() {
    let pi = 3.142;
    let sum = |n: f64, m: f64| -> f64{
        n + m
    };
    //Defining Closures
    let area_circleC = |rad: f64| -> f64{
        //Changing Into An Expression:
        pi * (rad * rad) + sum(10.0, 20.0)
    };
    println!("Using Closure: {}", area_circleC(3.0));
    println!("Using Function: {}", area_circleF(3.0));
}

//Function
//In Function Pi vairable Cannot Be Called
fn area_circleF(rad: f64) -> f64{
    3.14 * (rad * rad)
}
