// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// enum Shapes {
//     Square(String),
//     Triangle(String),
//     Circle(String),
// }

// let circle = Shapes::Circle(String::from("Circle"));

// let square = Shapes::Square(String::from("Square"));

// let home = IpAddr::V4(127, 0, 0, 1);

// let loopback = IpAddr::V6(String::from("::1"));

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}


fn main() {
    let penny_value = value_in_cents(Coin::Penny).to_string();
    let nickel_value = value_in_cents(Coin::Nickel).to_string();
    let dime_value = value_in_cents(Coin::Dime).to_string();
    let quarter_value: u8 = value_in_cents(Coin::Quarter(UsState::Alaska));
    // println!("{}", quarter_value);
    // let quarter_state = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", penny_value);
    println!("{}", nickel_value);
    println!("{}", dime_value);
    println!("{}", quarter_value);
    value_in_cents(Coin::Quarter(UsState::Alaska));

    // println!("{}", quarter_state);
}
