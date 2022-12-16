use std::io::Write;
fn main() {
    let a = ["LAGER: ", "STOUT: ", "NON-ALCOHOLIC "];
    let b = ["33 Export "," Legend ", " Maltina "];
    let c =   [" Desperados "," Turbo King ", " Amstel Malta "];
    let d = [" Goldberg ", " Williams ", " Malta Gold "];
    let e = [" Gulder ", " ", " Fayrouz "];
    let f = [" Heineken ", " ", " "];
    let g = [" Star \n", " \n", " \n"];
    
    let mut file = std::fs::File::create("project.txt").expect("create failed");
    file.write_all("Table for Nigerian Breweries\n".as_bytes()).expect("write failed");
    for i in 0..3{
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