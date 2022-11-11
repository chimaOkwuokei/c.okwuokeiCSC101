use std::io;
fn main() {
    //MENU
    println!("Poundo Yam/Edinkaiko Soup   -N3200 \n Fried Rice & Chicken   -N3000 \n Amala & Ewedu Soup    -N2500 \n Eba & Egusi Soup   - N2000 \n  White Rice & Stew   -N2500");
       
    println!("Enter your food order here");
    let mut food = String::new();
        io::stdin().read_line(&mut food).expect("Failed to read input");
         println!("Your food is: {}", food);
     
    println!("Enter your quantity here");
    let mut quan = String::new();
        io::stdin().read_line(&mut quan).expect("Failed to read input");
        let quan:f64 = quan.trim().parse().expect("Input not an integer");
        println!("Your quantity is: {}", quan);
   
   println!("Enter your price here");
    let mut p = String::new();
        io::stdin().read_line(&mut p).expect("Failed to read input");
        let p:f64 = p.trim().parse().expect("Input not an integer");
        println!("Your price is: {}", p);
   
   let d:f64 = quan * p;
   println!("Total price is: {}", d);
   
   if d > 10000.0 {
       println!("You have a discount of 5%,thus your new price is {}", d - d*(0.05) );
    }
}
