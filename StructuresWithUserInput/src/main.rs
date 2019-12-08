use std::io;

#[derive(Debug)]
struct Driver{
    name: String,
    gender: String,
    liscence: String,
}

impl Driver {
    fn asso_driver(name: String, gender: String, liscence: String) -> Driver{
        Driver{
            name,
            gender,
            liscence,
        }
    }
}
pub trait DriverType{
    fn Car_driver(&self) -> String;
    fn Truck_driver(&self) -> String;
}

impl DriverType for Driver{
    fn Car_driver(&self) -> String{
        format!("The Name Of The Driver Is: {}\nGender Is: {}\nLiscence type Is: {}", self.name, self.gender,self.liscence)
    }

    fn Truck_driver(&self) -> String{
        format!("The Name Of The Truck Driver Is: {}\nGender Is: {}\nLiscence Type Is: {}", self.name, self.gender, self.liscence)
    }
}


fn main(){
    let mut nam = String::new();
    println!("Enter Name Of The Driver: ");
    io::stdin().read_line(&mut nam).expect("Failed To Read Input");

    let mut gend = String::new();
    println!("Enter Gender Of The Driver: ");
    io::stdin().read_line(&mut gend).expect("Failed To Read Input");

    let mut lis_type = String::new();
    println!("Enter Liscense Type: ");
    io::stdin().read_line(&mut lis_type).expect("Failed To Read Input");


    let driver = Driver::asso_driver(
        nam, 
        gend,
        lis_type.trim().to_string(),
    );

    if driver.liscence == "LTV" || driver.liscence == "ltv"{
        println!("\nCAR DRIVER: \n{}", driver.Car_driver());
    }
    else if driver.liscence == "HTV" || driver.liscence == "htv" {
    println!("\nTRUCK DRIVER: \n{}", driver.Truck_driver());
    }
    else{
        println!("Invalid Liscense Type!");
    }
}