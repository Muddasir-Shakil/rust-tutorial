use std::{char::MAX, option, result, u8};

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

//Enum can have different variant defined with different datatypes assigned to each variant
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//Enums can have methods just as in structs

impl Message {
    fn call(&self) {}
}

// Match Control Flow Construct
//paterns that bind the values
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel, 
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn get_cents(&self) -> u8 {
        let mut ret_val: u8;
        match self {
            Coin::Penny => ret_val = 1,
            Coin::Nickel => ret_val = 5,
            Coin::Dime => ret_val = 10,
            Coin::Quarter(_state) => {
                ret_val = match _state {
                    UsState::Alabama => 1,
                    UsState::Alaska => 2,
                };
                println!("State quarter from {_state:?}!");
                ret_val = ret_val + 25
            }
        }
        println!("Got ${} cents", ret_val);
        ret_val
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn dice_roll (roll:i32) {
    match roll {
        3 => println!("got 3"),
        7 => println!("got 7"),
        _ => (),
    }
}


fn main() {
   
   // chapter 6.3
   let config_max: Option<u8> = Some(9);
   if let Some(max) = config_max {
    println!("The maximum value is set to {max}");
   }
   if let Some(8) = config_max  {
       println!("The max value we got is is 8");
   }
    //catch all Patterns and the _Placeholder
    dice_roll(3);
    dice_roll(7);
    dice_roll(9);
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);



    let mut c = Coin::Penny;
    c.get_cents();
    c = Coin::Dime;
    c.get_cents();
    c = Coin::Nickel;
    c.get_cents();
    c=Coin::Quarter(UsState::Alabama);
    c.get_cents();
    Coin::Quarter(UsState::Alaska).get_cents();
    let m = Message::Write(String::from("hello"));
    m.call();

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("${:?}  ${:?}", home, loopback);
}



// // match with variant data type enums 
// enum UaVariant {
//     UaBool(bool),
//     UaByte(u8),
//     Uafloat(f32),
//     UaDouble(f64),
//     UaString(String)
// }

// impl UaVariant {
//     fn create(&self) -> UaVariant{
//         match self {
//             UaVariant::UaBool()
//             UaVariant::UaByte(_) => todo!(),
//             UaVariant::Uafloat(_) => todo!(),
//             UaVariant::UaDouble(_) => todo!(),
//             UaVariant::UaString(_) => todo!(),        
//         }
        
//     }
// }

// struct VariantType {
//     value: UaVariant,
// }