fn main() {
    // Using Vec::new()
    let v : Vec<i64> = Vec::new();
    // printing the size of the vector
    println!("\n The length of the Vec::new is : {}", v.len());

    //using macro
    let v = vec!["Grace","Dan","Timi","Chima","Stino"];
    //printing the size of the vector 
    println!("\n The length of the vec macro is:{ }", v.len());
}
