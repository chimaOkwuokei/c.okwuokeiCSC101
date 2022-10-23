fn main(){
	let p1 = 210000;
	let r = 5;
	
	//but since this is depriciation,t=1 in all cases
	//for the first case
	let t = 1;
	let i1 = p1*r*t/100;
	let a1 = p1 - i1;
	println!("The amount(new principal) is {} after the first year", a1);
    
    let p2 = a1;
    let i2 = p2*r*t/100;
    let a2 = p2 - i2;
    println!("The amount(new principal) is {} after the second year", a2);

    let p3 = a2;
    let i3 = p3*r*t/100;
    let a3 = p3 - i3;
    println!("The amount(new principal) is {} after the third year", a3);
    println!("Therefore it depriciated to {}", a3 );
}
