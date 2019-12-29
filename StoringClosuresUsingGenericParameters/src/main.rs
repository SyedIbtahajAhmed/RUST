//Importing
use std::thread;
use std::time::Duration;
use std::io;
//Making A Structure
#[derive(Debug)]
struct Workout <T> 
    where T: Fn(u32) -> u32{
    //A Closure
    calculations_field: T,
    value: Option<u32>,
}

impl <T> Workout <T>
    where T: Fn(u32) -> u32{
    fn associate_workout(calculations_field: T) -> Workout<T>{
        Workout{
            calculations_field,
            value: None,
        }
    }

    //Another Fuction For Dropping the time Taken Repeatedly For Calculations
    fn the_value(&mut self, x: u32) -> u32{
        match self.value {
            Some(v) => v,
            //First Time This Block Will Run Because The Default Value In Value Field Is None
            None => 
                {   let v = (self.calculations_field)(x);
                    self.value = Some(v);
                    v
            }
        }
    }
}

//Making Function To Take Decision
fn workout_plan(nam: String, intensity: u32, random_number: u32){
    let mut calculations_workout = Workout::associate_workout(|inten| {
        //Doing Mathematical Calculations
        println!("\nDoing Mathematical Calculations. Please Wait A Little!");
        thread::sleep(Duration::from_secs(2));
        inten
    });
    //Decision Making
    if intensity > 30{
        println!("{}, Do {} PushUps.", nam, calculations_workout.the_value(intensity));
        println!("{}, Do {} SitUps.", nam, calculations_workout.the_value(intensity));
        println!("{}, Do {} PullUps.", nam, calculations_workout.the_value(intensity));
    }else{
        if random_number == 5{
            println!("Take A Rest!.");
        }else{
            println!("{}, Run {} Km!.", nam, calculations_workout.the_value(intensity));
        }
    }
}

fn main() {
    //Taking Name Input
    println!("Please Enter Your Name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed To Read Name!");
    let name: String = name.trim().parse().expect("Failed To Convert Name Into String!");

    //Intensity Input
    println!("Please Enter How Much Intensity Of Workout Do You Want? :");
    let mut intens = String::new();
    io::stdin()
        .read_line(&mut intens)
        .expect("Failed To Read Intensity!");
    let intens: u32 = intens.trim().parse().expect("Failed To Convert Intensity Into u32 Data Type!");

    //Random Number Input
    println!("Please Enter Any Random Number :");
    let mut random_num = String::new();
    io::stdin()
        .read_line(&mut random_num)
        .expect("Failed To Read Intensity!");
    let random_num: u32 = random_num.trim().parse().expect("Failed To Convert Random Number Into u32 Data Type!");

    //Calling The Function
    workout_plan(name, intens, random_num);
}