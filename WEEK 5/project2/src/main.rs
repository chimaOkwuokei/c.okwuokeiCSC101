use std::io;
fn main() {
    println!("Incentive Checker");

    //input experience
    println!("Are you experienced?");
    let mut e = String::new();
        io::stdin().read_line(&mut e).expect("Failed to read input");
        println!("Your experience level is: {}", e);
        if e == "Experienced"{
            println!("Welcome")
        }
        
    //input age
    println!("Enter your age.");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
        let age:i64 = age.trim().parse().expect("Input not an integer");
        println!("Your age is: {}", age);

    if age >= 40{
        println!("You're really enjoying, your incentive is N1,560,000.00" );
    }

    else if  age >= 30 && age < 40 {
        println!("You're enjoying, your incentive is N1,480,000.00" );
    }
    else if  age < 28 && age > 0{
        println!("You're good, your incentive is N1,300,000.00" );
    }
    
    else{
        println!("You're inexperienced, your incentive is N100,000.00" );
    }

}
