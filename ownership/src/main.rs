fn main() {
    let x = 5;
    let y = x;
    println!("x={},y={}",x,y);

    let s1 =String::from("hello");
    let s2 =s1;
    println!("{}, world! ", s2);

    let s1 =String::from("hello");
    let s2 =s1.clone();
    println!("{}, world!  {}",s1, s2);

    let s = String::from("hello");
    takes_ownership(s);
    //println!("{}",s);//value borrowed here after move


    let x:i32 = 5;
    makes_copy(x);
    println!("{}",x);

    println!("--------------------------");


    let s1 = gives_ownership();
    println!("s1 ownership {}",s1);


    let s2 = String::from("hello");

    let s3  = tasks_and_gives_back(s2);
    println!("{}",s3); //s2 value borrowed here after move


    let s1 = String::from("hello");
    let (s2,len) = calculate_length(s1);
    println!("The length of '{}' is {} .", s2, len);


}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
fn tasks_and_gives_back(a_string:String) -> String {
  a_string
}
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}
fn takes_ownership(some_string:String) {
    println!("{}",some_string);
}

fn makes_copy(some_integer:i32) {
    println!("{}",some_integer);
}

