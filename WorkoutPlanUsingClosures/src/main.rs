use std::thread;
use std::time::Duration;
use std::io;

//Function Tha Will Do Calculation Of Workout (Backend Code)
// fn expensive_calculation(num: u32) -> u32{
//     println!("\nDOING MATHEMATICAL CALCULATION! PLEASE WAIT...");
//     thread::sleep(Duration::from_secs(2));      //2 Seconds Delay
//     num
// }

//Defining Structure To Store Workout Plan From The backend Calculation
#[derive(Debug)]
struct Cacher <T> 
//Every Closure Implement Fn Trait
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

//Implementing Structure (Associate Function):
impl <T> Cacher <T> 
    where T: Fn(u32) -> u32
{
    fn Associate_cacher(calculation: T) -> Self{
        Self{
            calculation,
            value: None,
        }
    }

    //Another Method
    fn the_value(&mut self, x: u32) -> u32{
        match self.value{
            //After First Time None Variant Will Not Run
            Some(v) => v,
            //First Time None Variant Will Run
            None => {
                let v = (self.calculation) (x);
                self.value = Some(v);
                v
            }
        }
    }
}


//Functions That Will Direct To expenive_calculations Funcions (App Code Decision Making Code):
fn generate_workotuplan(intensity: u32, random_number: u32) {

    //Defining The Same Function Using A Closure And Storing That Closure In a Structure
    let mut expensive_calculation = Cacher::Associate_cacher(|num| {
        println!("\nDOING MATHEMATICAL CALCULATION! PLEASE WAIT...");
        thread::sleep(Duration::from_secs(2));      //2 Seconds Delay
        num
    });
    //Desicion Making And Results
    if intensity < 25 {
        println!("Do {} SitUps!", expensive_calculation.the_value(intensity));
        println!("Do {} PushUps!", expensive_calculation.the_value(intensity));
    }else{

        if random_number == 5 {
            println!("Take Rest!");
        }else{
            println!("Run {} Km!", expensive_calculation.the_value(intensity));
        }
    }
}

fn main() {
    //User Defined Intensity
    let mut user_intensity = String::new();
    io::stdin().read_line(&mut user_intensity).expect("Failed To Read Input!");
    let user_intensity: u32 = user_intensity.trim().parse().expect("Failed To Read Input!");

    //User Defing Random Number
    let mut user_random_number = String::new();
    io::stdin().read_line(&mut user_random_number).expect("Failed To Read Input!");
    let user_random_number: u32 = user_random_number.trim().parse().expect("Failed To Read Input!");

    //generating Workout Plan
    generate_workotuplan(user_intensity, user_random_number);
}
