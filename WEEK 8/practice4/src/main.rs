fn main() {
    let name = vec!["Grace","Dan","Timi","Chima","Stino"];
    let age  = vec![16,17,19,24,26];
    print!("\nAge Allocation:\n");
    for i in 0..name.len(){
        println!("{} is {} years old!\n", name[i], age[i] );
    }
}
