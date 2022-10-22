fn main(){
	let p = 1000;
	let r = 4;
	let n = 2;
	let a = p*((1 + (r/100))*n);
	println!("The amount is {}", a );

	//I = A - P
	let i = a - p;
	println!("The simple interest is {}", i );
}