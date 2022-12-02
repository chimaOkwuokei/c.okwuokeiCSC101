use std::io;
fn main() {
    let mut city : Vec<String> = Vec::new();
    println!("The City vector has element {}", city.len() );
    
    let mut input1 = String::new();
    println!("How many cities do you want to enter?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num:i32 = input1.trim().parse().expect("Invalid input");
    
    for i in 0..city_num{
    let mut input2 = String::new();
    println!("Enter City{}", i+1);
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let new_city:String = input2.trim().parse().expect("Invalid input");
    city.push(new_city);
    }
    print!("Your preferred cities are:\n");
    let mut i = 1;
    for j in city{
        println!("{} {}",i, j );
        i+=1;
    }
}
