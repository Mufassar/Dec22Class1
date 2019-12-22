
#[derive(Debug)]
struct Student
{
    name:String,
    score:u32
}

#[derive(Debug)]
struct Team
{
    country:String,
    score: u16
}

#[derive(Debug)]
struct Food
{
    name:String,
    price:u16,
    serving:u8
}

impl Food
{
    fn create_food(name: String, price: u16, serving: u8)-> Food
    {
        return Food {name, price, serving};
    }       
}

impl Team
{
    fn team_score(&self) -> u16
    {
        self.score
    }

    fn high(&self, other: &Team)
    {
        if self.score > other.score 
        {
            println!("Team 1 has high score");
        }
        else
        {
            println!("Team 2 has high score");
        }
    }

    fn high_score_team(&self, second: &Team) -> Team
    {
        let first = Team {country: self.country.to_string(), score: self.score};
        let second = Team {country: second.country.to_string(), score: second.score};

        if first.score > second.score 
        {
            return first;
        }
        else
        {
            return second;
        }        
    }
}

impl Student
{
    fn details(&self)
    {
        println!("Struct of student is {:#?}", self);
        
        println!("The name of student in method is {} & his score is {}", self.name, self.score);
    }

    fn recieve_score(&self) -> u32
    {
        return self.score;
    }
}

fn main() {
    
    println!("Class 22 Dec 2019");

    let student1 = Student { name:"Nouman".to_string(), score:70 };    

    student1.details();// Struct Method Calling

    details(&student1); // Function Calling

    let student_score = student1.recieve_score();

    println!("Calling method recieve_score {}", student_score);

    let team1 = Team {country: "Pakistan".to_string(), score: 400 };

    let team1_score = team1.team_score();

    println!("The score of team 1 is {}", team1_score);

    let team2 = Team {country: "Srilanka".to_string(), score: 200 };

    let team2_score = team2.team_score();

    println!("The score of team 2 is {}", team2_score);

    team1.high(&team2);

    let returned_team = team1.high_score_team(&team2);

    println!("The winning team is {:#?}", returned_team);

    let food_name = "Pasta".to_string();
    let food_price = 100;
    let food_serving = 2;

    let food1 = create_food(food_name, food_price, food_serving);

    println!("The details of {:#?}", food1);

    let food_name = "Pasta".to_string();

    let food2 = Food::create_food(food_name, food_price, food_serving);

    println!("The details of food from associated function {:#?}", food2);
}

fn details(student: &Student)
{
    println!("{:#?}", student);
}

fn create_food(name: String, price: u16, serving: u8)-> Food
{
    return Food {name, price, serving};
}
