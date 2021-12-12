use rand::Rng;

fn main() {
    // CONSTANTS
    const C_KM_S: i32 = 299_789; // km/s
    println!("Speed of light is {} km/s", C_KM_S);

    // SHADOWING
    let x = 32;
    let x = x + 1;
    let x = x - 2;
    let mut x = x - x;
    x += 1;
    println!("Values of x is {}", x);

    // Also 0.30000000000000004
    println!("Result: {}", 0.1 + 0.2);

    // TUPLES
    let tup = ("string", 4, 0.32);

    let (a, b, c) = tup;
    println!("The tuple's values are: {}, {} and {}", a, b, c);
    println!("And using dot notation: {}, {} and {}", tup.0, tup.1, tup.2);

    // ARRAYS
    let _arr = [1, 2, 3];
    let _arr: [i32; 3] = [1, 2, 3];
    let _arr: [i32; 100] = [-1; 100];
    // arrays are not only immutable in reference, but also in content; the next line will fail
    // _arr[3] = 2;
    println!("Element at position 3: {}", _arr[3]);

    // STATEMENTS AND EXPRESSIONS
    // STATEMENTS RETURN NOTHING
    // EXPRESSIONS DO
    // variable assignments are statements
    let x = 3;
    let a = "a string";

    // code blocks without ending semicolon are expressions
    let y = {
        let x = 3;
        x + 1
    };

    println!("The values are: x = {}, a = {}, y = {}", x, a, y);

    // FUNCTIONS
    println!("Five: {}", five());
    println!(
        "1 or 2: {}, {}, {}, {}, {}",
        one_or_two(),
        one_or_two(),
        one_or_two(),
        one_or_two(),
        one_or_two()
    );

    // LOOPS

    // loop, while and for
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // break is an expression, so the value after it will be returned
            break counter * 2;
        }
    };

    println!("counter: {}; result: {}", counter, result);

    // not much to say about while loops, pretty straightforward stuff

    let array = [1, 2, 3, 4, 5];
    for element in array.iter() {
        println!("The value is {}", element);
    }

    for number in (1..4).rev() {
        println!("Countdown: {}", number);
    }

    println!("This is the end of the program!")
}

// functions return implicitly the value of the last expression
fn five() -> i32 {
    5
}

fn one_or_two() -> i32 {
    let rnd = rand::thread_rng().gen_ratio(1, 2);

    if rnd {
        return 1;
    }
    2
}
