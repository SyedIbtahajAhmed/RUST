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
        format!("The Name Of The Driver Is: {}Gender Is: {}Liscence type Is: {}", self.name, self.gender,self.liscence)
    }

    fn Truck_driver(&self) -> String{
        format!("The Name Of The Truck Driver Is: {}Gender Is: {}Liscence Type Is: {}", self.name, self.gender, self.liscence)
    }
}


fn main(){
    println!("How Many Drivers You Want To Add: ");
    let mut driv = String::new();
    io::stdin().read_line(&mut driv).expect("Failed To Read the Input!");
    let mut driv: i32 = driv.trim().parse().expect("Failed To Read!");
    
    let mut driver: Vec<Driver> = Vec::new();

    for _i in 0..driv{
        let mut nam = String::new();
        println!("Enter Name Of The Driver: ");
        io::stdin().read_line(&mut nam).expect("Failed To Read Input");

        let mut gend = String::new();
        println!("Enter Gender Of The Driver: ");
        io::stdin().read_line(&mut gend).expect("Failed To Read Input");

        let mut lis_type = String::new();
        println!("Enter Liscense Type: ");
        io::stdin().read_line(&mut lis_type).expect("Failed To Read Input");


        driver.push(Driver::asso_driver(
            nam, 
            gend,
            lis_type.trim().to_string(),
        )
        );
    }

    for i in &driver{
        if i.liscence == "LTV" || i.liscence == "ltv"{
            println!("\nCAR DRIVER: \n{}", i.Car_driver());
        }
        else if i.liscence == "HTV" || i.liscence == "htv" {
        println!("\nTRUCK DRIVER: \n{}", i.Truck_driver());
        }
        else{
            println!("Invalid Liscense Type!");
        }
    }
}