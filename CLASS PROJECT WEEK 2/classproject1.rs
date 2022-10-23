fn main(){
	let p = 520000000;
	let r = 10;
	let n = 5;
	//therefore A is;
	let a = p*(1 + (r/100))*n;
	println!("The amount is {}", a);
	//therefore CI is;
	let ci = a - p;
	println!("The CI is {}", ci);

}