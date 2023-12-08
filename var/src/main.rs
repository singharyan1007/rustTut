fn main() {
    let x=6;
    println!("The value of x is {x}");
 
    let mut y=69;
 
    println!("{y}");
    y=90;
    println!("{y}");
 
 
    //constants
    const THREE_SIX:u32=60*30;
    println!("{THREE_SIX}");
 
    //shadowing
    let z=5;
    let z=z+1;
    println!("Print the outer z : {z}");
    {
     let z=z*2;
     println!("Print the inner scope z : {z}");
    }
 
    let spcs=" ";
    let spcs=spcs.len();
    println!("{spcs}")
 
 
 }
 