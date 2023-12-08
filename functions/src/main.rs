fn main() {
    println!("Hello, world!");
    let my_string=String::from("Aryan");

    name_print(&my_string);

    // let x=five();
    let x = plus_one(5);
    println!("{x}");

}

fn name_print(name:&String){
    println!("Hi, may name is {name}")
}


fn five() -> i32{
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
