fn main() {
//    let sum:f32=20.98;
//    let sum:bool=true;
//    println!("{sum}");

//    let puppy_faced_value:char='🐕';
//    println!("{puppy_faced_value}");

   //Tuples
   let tup=(300.0,356,false);
   let (ar_x,ay_y,az)=tup;
   println!("{ar_x}");
   println!("{ay_y}");
   println!("{az}");

   //Accessing the tuple values directly using the dot(.) oprator
   let three_hundred=tup.0;
   println!("{three_hundred}");
}
