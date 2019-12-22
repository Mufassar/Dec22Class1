
#[derive(Debug)]
struct Student
{
    name:String,
    score:u32
}

impl Student
{
    fn details(&self)
    {
        println!("The name of student in method is {} & his score is {}", self.name, self.score);
    }
}

fn main() {
    
    println!("Class 22 Dec 2019");

    let mut student1 = Student { name:"Nouman".to_string(), score:70 };    

    student1.details();// Struct Method Calling

    details(student1); // Function Calling
}

fn details(student: Student)
{
    println!("{:#?}", student);
}
