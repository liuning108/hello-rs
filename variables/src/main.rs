const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {

    let mut  x =5;
    println!("The value of x is {}",x);
    x =6;
    println!("The value of x is {}",x);
    println!("3 HOURS IN Seconds:{}",THREE_HOURS_IN_SECONDS);


    //Shadowing

    let y = 5;
    let y = 5+1; // shadowing
    {
        let  y = y *2;
        println!("The value of y in the inner scope is {}",y);
    }

    println!("The value of y is {}",y);

    let f = 2.0;


}