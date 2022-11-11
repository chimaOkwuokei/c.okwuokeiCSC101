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

    let s:f64 = (a + b + c);
    let mut area:f64 = s * (s - a) * (s - b) * (s - c);
    area = area.sqrt();
 
    println!("Area of a triangle: {}", area);
}
