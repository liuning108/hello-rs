
fn main() {
    let mut s1 =String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}",s1,len);
    change(&mut s1);
    println!("The length of s1 is {}",s1);


    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    }
    let _r2 = &mut s;

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    let r3 = &mut s;

    println!("{}", r3);

    let reference_to_nothing = dangle();
    println!("{}",reference_to_nothing);

}
fn dangle()->String {
    let s = String::from("hello");
    s
}

fn change(some_string:&mut String) {
    some_string.push_str(",world");
}
fn calculate_length(s:&String) -> usize {
    s.len()
}

