use std::io;
use std::io::Read;
fn admin() {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn project_manager() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn employee() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn customer() {
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn vendor() {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}


fn main(){
    let mut value = String::new();
    println!("\nEnter your 1 if you are an Administrator \n Enter your 2 if you are a Project Manager \n Enter your 3 if you are an Employee\n Enter your 4 if you are a Customer \n Enter your 5 if you are a Vendor ");
    io::stdin().read_line(&mut value).expect("Failed to read input");
    let a:i32 = value.trim().parse().expect("Invalid input");
   
   if a == 1{
        admin();
    }
    else if a == 2{
        project_manager();
    }
    else if a == 3{
        employee();
    }
    else if a == 4{
       customer();
    }
    else if a == 5{
       vendor();
    }
    else{
        print!("You entered a wrong value");
    }
}
