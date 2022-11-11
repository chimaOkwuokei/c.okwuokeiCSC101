use std::io;
fn main() {
    println!("Quadractic Formula yipee");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a;");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b;");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c;");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    let d:f64 = (b * b) - (4.0 *a * c);
    let p = f64::powf(d,0.5);
    let x1:f64 = (-b + p) /2.0*a;
    let x2:f64 = (-b - p)/2.0*a;
    println!("The determinant is: {}", d);
    println!("x1 is: {}", x1);
    println!("x2 is: {}", x2);

    if d > 0.0{
        println!("There are two distinct roots!");
    }
    if d == 0.0 {
        println!("There is exactly one real root!");
    }
    if d < 0.0{
        println!("There are no real roots!");
    }
}
