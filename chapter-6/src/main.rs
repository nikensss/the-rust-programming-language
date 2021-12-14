fn main() {
    println!("Hello, world!");

    let four = IPAddrKind::V4(String::from("localhost"));
    let six = IPAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    let quit = Message::Quit;
    quit.call();

    let move_msg = Message::Move { x: 3, y: 4 };
    move_msg.call();

    let write = Message::Write(String::from("Saving data!"));
    write.call();

    let change_color = Message::ChangeColor(123, 48, 189);
    change_color.call();

    let coins: [Coin; 4] = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter(None)];
    for coin in coins {
        println!("{:?}: {}", coin, coin.value_in_cents())
    }

    let quarters: [Coin; 3] = [
        Coin::Quarter(None),
        Coin::Quarter(Some(UsState::Alabama)),
        Coin::Quarter(Some(UsState::Alaska)),
    ];

    for quarter in quarters {
        println!("Has state? {}", quarter.has_state())
    }
}

#[derive(Debug)]
enum IPAddrKind {
    V4(String),
    V6(String),
}

fn route(ip: IPAddrKind) {
    println!("Routing IP {:?}", ip)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match &self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(msg) => println!("Writting: {}", msg),
            Message::ChangeColor(r, g, b) => println!("Color is now: rgb({}, {}, {})", r, g, b),
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<UsState>),
}

impl Coin {
    fn value_in_cents(&self) -> i32 {
        match &self {
            Coin::Dime => 10,
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Quarter(_state) => 25,
        }
    }

    fn has_state(&self) -> bool {
        // we can match to any depth: variant of Coin, variant of Option, variant of UsState
        // if let Coin::Quarter(Some(UsState::Alaska)) = &self {
        //     println!("Alaska coin!")
        // }

        if let Coin::Quarter(Some(_state)) = &self {
            return true;
        }
        false

        // can also be done with a match on &self, but it is a bit longer
        // match &self {
        //     Coin::Quarter(state) => match state {
        //         Some(_us_state) => true,
        //         None => false,
        //     },
        //     _ => false,
        // }
    }
}
