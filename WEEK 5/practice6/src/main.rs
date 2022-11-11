use std::io;

fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter lower bound:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let l:i32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter upper bound:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let u:i32 = input2.trim().parse().expect("Not a valid number");

   for x in l..u{
    println!("Count Level is {}", x );
   }
}