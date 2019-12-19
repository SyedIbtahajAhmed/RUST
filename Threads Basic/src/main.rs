//Closures (functional Programming)
//A Function Strored In A Variable Is Called Closure
use std::thread;
use std::time::Duration;
//That App Will Call This Function
fn simulated_expensive_calc(intensity: u32) -> u32{
    println!("Doing MAthematical Calculations!");
    thread::sleep(Duration::from_secs(2));    //Delay Of Two Seconds
    intensity
}

//App Code
//this Code Will Call The simulated_expensive_calc Function To Do Mathematical Calculation
fn generate_workout(intensity: u32){
    if intensity < 30 {
        println!("Do {} Push Ups!", simulated_expensive_calc(intensity));
        println!("Do {} Sit Ups!", simulated_expensive_calc(intensity));
    }
    else{
        println!("Run {} Meters.", simulated_expensive_calc(intensity * 4));
    }
}

fn main() {
    let inten = 45;
    generate_workout(inten);
}