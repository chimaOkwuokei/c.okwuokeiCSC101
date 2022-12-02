use std::io;

fn checker() {

    let mut input = String::new();
    println!("Enter a character");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a:char = input.trim().parse().expect("Invalid input");

    if a >= '0' && a <= '9'{
        println!("Character '{}' is a digit", a);
    }
    else{
        println!("Character '{}' is not a digit ", a );
    }
}
fn main(){
    println!("Welcome! This program checks whether a character variable contains a digit or not ");
    checker()
}
