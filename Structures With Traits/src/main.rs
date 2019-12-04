#[derive(Debug)]
struct Student{
    first_name: String,
    last_name: String,
    age: u8,
    address: String,
}


//Defining Trait For The Object(Student)
pub trait Information{
    fn only_first_name(&self) -> String;
}

impl Information for Student{
    fn only_first_name(&self) -> String{
        let std1 = format!("\nName: {}",
            self.first_name);
            std1
    }
}


impl Student{
    pub fn create_student(first_name: String, last_name:String, age: u8, address: String) -> Student{
        Student{
            first_name,
            last_name,
            age,
            address,
        }
    }

    //Working same as trait function
    // fn view_std(&self) -> String{
    //     let std1 = format!("Name: {}, Age: {}, Address: {}",
    //         self.name,
    //         self.age,
    //         self.address);
    //         std1
    // }
}

fn main(){
    let student1 = Student::create_student(
        "Syed".to_string(), 
        "Ibtahaj".to_string(),
        19, 
        "syd.ibthaj@gmail.com".to_string());

    println!("{}", student1.only_first_name());
    // println!("{}", student1.view_std());
    // println!("{}", student1.name);

    // let student2 = Student::create_student(
    //     String::from("Syed Saboor"), 
    //     20, 
    //     String::from("syed.saboor@gmail.com"));

    // println!("{:?}", student2);
    // println!("{}", student2.view_std());
}