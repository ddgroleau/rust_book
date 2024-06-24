fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }

    println!("The value of x is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("{spaces}");

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("{guess}");

    let int:i32 = 1;
    let string:&str = "OK";
    let float:f64 = 2.25;
    let boolean:bool = true;
    let character:char = 'A';

    println!("{int} {string} {float} {boolean} {character}");

    let tup = (500, 6.4, 1);

    let (_, z, _) = tup;

    println!("The value of y is: {z}");

    let tup2: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup2.0;

    let six_point_four = tup2.1;

    let one = tup2.2;

    println!("{five_hundred} {six_point_four} {one}");

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("{}",months[0]);

    another_function(12,'a');

    println!("{}",arrow());

    /* 
        This is a comment
    */
    // This is a comment

    let number = 3;

    if number < 5 {
        println!("< 5");
    } else if number > 4 {
        println!("> 4");
    } else {
        println!("Default")
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{number}");

    let mut count = 1;
    let result = loop {
        if count > 3 {
            break count;
        }
        println!("{count} again!");
        count += 1;
    };
    println!("{result}");
    nested_loops();

    while_loop();
    for_loop();
}


fn another_function(x:i32, y:char) {
    println!("Another function {x}{y}.");
}

fn arrow() -> i32 { 1 }

fn nested_loops() -> i32 {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    count
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}