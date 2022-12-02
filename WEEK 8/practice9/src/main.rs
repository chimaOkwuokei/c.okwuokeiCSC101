fn main() {
    let b:(i32,bool,f64) = (17,true,4.75);
    print(b);
}

fn print(x:(i32,bool,f64)){
    println!("Inside the print method");
    let (age,is_male,cgpa) = x;
    println!("Age is {}, ismale? {}, cgpa is {}", age,is_male,cgpa);
}
