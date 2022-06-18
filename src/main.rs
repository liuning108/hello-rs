fn main() {
    let y = {
        let x = 1;
        x+2
    };
    println!("y is {}",y);
    println!("five function is {}",five());
    println!("five function is {}",plus_one(19));


}

fn plus_one(x:i32)-> i32 {
    x+1;

}

///no semicolon beacause it's an expression
/// whose value want to return
fn  five() -> i32 {5}