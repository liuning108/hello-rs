fn main() {

    let mut  s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
   // s.clear();
    println!("{}",word);

    let  s = String::from("hello world");
    let  mut  hello = &mut s[0..5].to_string();
    let world = &s[6..11];
    hello.clear();
    println!("{},{}",s,world);


    let mut s = String::from("hello world");
    let  mut  hello =&mut s;
    hello.clear();
    println!("{}",s);
}



fn first_word(s:&String) -> &str {
    let bytes = s.as_bytes();
    for (i ,&item) in bytes.iter().enumerate() {
         if item == b' ' {
             return  &s[0..i]
         }
    }
    &s[..]
}