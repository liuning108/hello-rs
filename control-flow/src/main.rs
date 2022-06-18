fn main() {
    let number =7;
    //must be explicit boolean value;
    if  number != 0 {
        println!("condition was true");
    }
    let condition = true;


    // type must be are incompatible
    let number = if condition{5} else { 6 };
    println!("The value of number is :{}",number);


    let mut count =0;
    'counting_up:loop {
        println!("count={}",count);
        let mut remaining = 10;
        loop {
             println!("remaining = {}",remaining);
            if remaining==9{
                break;
            }
            if count == 2{
                break 'counting_up
            }
            remaining -=1;
        }
        count +=1;
    }



    let mut counter = 0;

    let result = 'outlook:loop {
        counter +=1;

        loop {
            if counter== 10 {
                break 'outlook counter*2;
            }
            println!("inner loop");
            break;
        }

    };
    println!("The result is {}", result);


    let a = [10,20,30,40,50];
    let mut  index =0;
    while index<5 {
        println!("the value is :{}",a[index]);
        index+=1;
    }

    for element in a {
        println!("the value is: {}", element);
    }


    for number in (1..=4).rev() {
        println!("the value is: {}", number);
    }
    println!("LIFTOFF!!!");

}
