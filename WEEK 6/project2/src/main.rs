use std::io;
fn checker(){
  let clients_arr = ["Chima", "Desmond", "Makuo", "Somto", "Pius"];
  
  for i in 0..5{
    println!("{}, how many siblings do you have?",clients_arr[i] );
    let mut input1 = String::new();
    println!("Enter the amount of siblings you have:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Invalid input");//a stands for number of siblings!
    println!("{} sibling(s)", a);
   
  for j in 0..a {
  
    println!("Enter his/her name");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input");
    println!("Name: {}", n);
    
    println!("Enter the age:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age:i64 = input2.trim().parse().expect("Invalid input");
    println!("Age: {}", age);

    if age > 18{
        println!("Enter 1 for married and enter 0 for single.");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let x:i64 = input3.trim().parse().expect("Invalid input");
 /*1 for single*/
        if x == 1{
        println!("Do you have any offspring(s)? What city does your family reside?");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        println!("{}",input4 );
        }
        else if x == 0{
        println!("Enter 1 if you're a worker and 0 if you're a student");
        let mut input5 = String::new();
        io::stdin().read_line(&mut input5).expect("Failed to read input");
        let y:i64 = input5.trim().parse().expect("Invalid input");

        if y == 0{
        println!("Kindly enter your University and course of study.");
        let mut z = String::new();
        io::stdin().read_line(&mut z).expect("Failed to read input");
        println!("My University and course of study are:{}", z );
        
        }
        }
        }
    else if age < 18{
        println!("What is your WAEC status? Input 1 for yes and 0 for no.");
        let mut input6 = String::new();
        io::stdin().read_line(&mut input6).expect("Failed to read input");
        let b:i64 = input6.trim().parse().expect("Invalid input");
         if b == 1{
            println!("Input your secodary school attended.");
            let mut sec = String::new();
            io::stdin().read_line(&mut sec).expect("Failed to read input");
            println!("I attended {}",sec );
         }
         else{
             println!("Input your current class level.");
            let mut class = String::new();
            io::stdin().read_line(&mut class).expect("Failed to read input");
            println!("I am in {}",class );
         
    }
   
    }
   }

  }

}

fn main() {
    checker()
}
