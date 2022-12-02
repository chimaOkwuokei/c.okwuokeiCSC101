fn main() {
    let numbers = [1, 2, 3, 4, 5];
    println!("Original array = {:?}", numbers);

    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced {:?}", slice1 );

    let slice2 = &numbers[..3];
    println!("index 0 to index 3 is sliced {:?}", slice2 );
    
    let slice3 = &numbers[2..];
    println!("index 2 to index 5 is sliced {:?}", slice3 );
   
    let slice4 = &numbers[..];
    println!("index 0 to index 5 sliced {:?}", slice4 );
//always exclusive of the last index
}
