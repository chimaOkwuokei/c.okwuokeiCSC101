use std::io;
fn calc(){
  println!("\n If you want to calculate the area of the following,input it's corresponding values;\n Trapezium - 1 \n Rhombus - 2 \n Parallelogram - 3 \n Cube - 4 \n Cylinder - 5\n");

    println!("Which task?");
    let mut e = String::new();
    io::stdin().read_line(&mut e).expect("Failed to read input");
    let e:f32 = e.trim().parse().expect("Invalid input");
    println!("The task is {}", e);

    //TRAPEZIUM
    if e == 1.0 {
        println!("You are dealing with a Trapezium");
    let mut input1 = String::new();
    println!("Enter the value of height");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let h:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter the value of b1");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b1:f32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter the value of b2");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let b2:f32 = input3.trim().parse().expect("Invalid input");
    
    let a = h/2.0*(b1+b2);
     println!("The area of the trapezium is {}", a);
    } 
    //RHOMBUS
    else if e == 2.0{
       println!("You are dealing with a Rhombus");
    let mut input1 = String::new();
    println!("Enter the value of d1");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let d1:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter the value of d2");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let d2:f32 = input2.trim().parse().expect("Invalid input");
    
    let a = 0.5 * d1 * d2;
     println!("The area of the rhombus is {}", a);
    }
    //PARALLELOGRAM
    else if e == 3.0{
       println!("You are dealing with a Parallelogram");
    let mut input1 = String::new();
    println!("Enter the value of the base");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter the value of the altitude");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let alt:f32 = input2.trim().parse().expect("Invalid input");
    
    let a = base * alt;
     println!("The area of the Parallelogram is {}", a);
    }
    //CUBE
     else if e == 4.0{
       println!("You are dealing with a Cube");
    let mut input1 = String::new();
    println!("Enter the value of the length");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let l:f32 = input1.trim().parse().expect("Invalid input");
    
    let a = 6.0 * l * l;
     println!("The area of the rhombus is {}", a);
    }
    //CYLINDER
     else if e == 5.0{
       println!("You are dealing with a Cylinder");
    let mut input1 = String::new();
    println!("Enter the value of the radius");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let r:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter the value of the height");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let h:f32 = input2.trim().parse().expect("Invalid input");
    
    let a = 3.142 * r * r * h;
     println!("The area of the cylinder is {}", a);
    }
    else{
      println!("Wrong value inputed!");
    }
}

fn main() {
    calc();
}
