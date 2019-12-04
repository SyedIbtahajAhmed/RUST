//Three Stages
//Development
//Debug
//Production

//Defining Struct
#[derive(Debug)]
struct Car{
    engine: i32,
    seats: i32,
    name: String,
}
//Implementing Struct
impl Car{
    //Associated Function
    fn asso_car(engine: i32, seats: i32, name: String) -> Car{
        Car{
            engine: engine,
            seats: seats,
            name: name,
        }
    }
    //Method
    fn meth_car(&self) -> String{
        if self.engine >= 1300 {
            String::from(format!("{} Would Be More Then 2 Million!", self.name))
        }
        else{
            String::from(format!("{} Car Would Be Less Then 2 Million!", self.name))
        }
    }
}

fn main(){
    let obj1 = Car::asso_car(
        1300,
        4,
        String::from("Civic"),
    );


    println!("{:?}", obj1.meth_car());
    println!("{:?}", obj1);
}