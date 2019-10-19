// Student Struct
#[derive(Debug)]
struct Student {
    name:String,
    grade: char,
    age:u8,
    percentage: f32,
}
impl Student {
    fn new() -> Student {
        Student {
            age: 25,
            name: String::from("Ahmed"),
            grade: 'A',
            percentage: 80.0,
        }
    }
}
impl Student {
    fn percentage(&self)-> f32 {
        self.percentage
    }
}
fn main(){
    let student1 = Student::new();
    println!("{:#?}",student1);

    println!("The Percentage of the Student is : {}%",student1.percentage());
}