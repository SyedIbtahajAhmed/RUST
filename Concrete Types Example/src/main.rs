#[derive(Debug)]
struct Vehicle{
    EngineCC: String,
    Color: String,
    Model: String,
}

impl Vehicle{
    fn create_veh(eng: String, col: String, model: String) -> Vehicle{
        Vehicle{
            EngineCC: eng,
            Color: col,
            Model: model,
        }
    }
}

pub trait Vehicle_Type{
    fn Car(&self) -> String;
    fn Truck(&self) -> String;
}

impl Vehicle_Type for Vehicle{
    fn Car(&self) -> String{
        format!("Engine CC: {}\nColor Is : {}\nModel Is : {}", self.EngineCC, self.Color, self.Model)
    }

    fn Truck(&self) -> String{
        format!("Engine CC: {}\nColor Is : {}\nModel Is : {}\nLoading: True", self.EngineCC, self.Color, self.Model,)
    }
}

fn main() {
    let v1 = Vehicle::create_veh(
        "1300".to_string(), 
        "red".to_string(), 
        "2019".to_string()
    );
    let v2 = Vehicle::create_veh(
        "2300".to_string(), 
        "Blue".to_string(), 
        "2020".to_string()
    );    
    println!("The First Vehicle Type Is: \n{}",v1.Car());
    println!("The Second Vehicle Type Is: \n{}",v2.Truck());
}