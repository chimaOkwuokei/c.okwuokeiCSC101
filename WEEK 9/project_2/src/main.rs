use std::io::Write;
fn main() {
    let a = ["", "", "PAU SMIS", " "];
    let b = ["STUDENT NAME "," MATRIC NUMBER", " DEPARTMENT "," LEVEL " ];
    let c = [" Oluchi Mordi"," ACC10211111 ", " Accounting", " 300 "];
    let d = [" Adams Aliyu ", " ECO10110101", " Economics ", " 100 "];
    let e = [" Shania Bolade", " CSC10328828 ", " Computer ", " 200 "];
    let f = [" Adekunle Gold ", " EEE11020202 ", " Electrical "," 200 "];
    let g = [" Blanca Edemoh\n", " MEE10202001 \n", " Mechanical \n"," 100\n"];
    for i in 0..4{
        println!("{}", a[i]);
        println!("{}", b[i]);
        println!("{}", c[i]);
        println!("{}", d[i]);
        println!("{}", e[i]);
        println!("{}", f[i]);
        println!("{}", g[i]);
    }
    let mut file = std::fs::File::create("project.csv").expect("create failed");
    file.write_all("STUDENTS DETAILS\n".as_bytes()).expect("write failed");
    for i in 0..4{
    file.write_all(a[i].as_bytes()).expect("write failed");
    file.write_all(b[i].as_bytes()).expect("write failed");
    file.write_all(c[i].as_bytes()).expect("write failed");
    file.write_all(d[i].as_bytes()).expect("write failed"); 
    file.write_all(e[i].as_bytes()).expect("write failed");
    file.write_all(f[i].as_bytes()).expect("write failed");
    file.write_all(g[i].as_bytes()).expect("write failed");
          }
    println!("\n Data written to file.");
    

}