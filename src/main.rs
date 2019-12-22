
#[derive(Debug)]
struct Student
{
    name:String,
    score:u32
}

fn main() {
    
    println!("Class 22 Dec 2019");

    let student1 = Student { name:String::from("Nouman"), score:70 };


    details(student1);
}

fn details(student: Student)
{
    println!("{:#?}", student);
}
