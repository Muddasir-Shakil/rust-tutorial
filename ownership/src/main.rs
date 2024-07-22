
fn main() {
    // references();
    // danglin_ref();
    // slices();
    structs();
}

fn structs()  {
  let mut user1= User {
    active: true,
    user_name: String::from("user1.1"),
    email: String::from("user@hotmail.de"),
    sign_in_count: 10
  }; 

  // ".." is an update syntax, updating user2 from the values of user1
  let user2 = User {
    email: String::from("emailId"),
    ..user1
  };

  println!("{0}", user1.user_name);

}

fn structFieldInitShorthand(email: String, user_name: String) -> User {
  return User {
    active: true,
    user_name,
    email,
    sign_in_count:10
  }
}

struct User {
  active: bool, 
  user_name: String, 
  email: String,
  sign_in_count: u64
}

fn slices() {
  let s = String::from("Hello World");
  
  let word = first_word(&s[0..6]);

  println!("{word}");
  let word = first_word(&s[6..]);

  println!("{word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    // for &byte in bytes  {

    // }

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


// fn references() {
//     let mut s = String::from("hello");
//     let r1 = &s;
//     let r2 = &s;
//     let r3 = &mut s;

//     println!("{r3}");

//     takes_ownership(&s);
//     takes_ownership(&s);
//     change(&mut s);
//     change(&mut s);
//     let x = 5;

//     makes_copy(x);
//     makes_copy(x);
// }

// fn takes_ownership(some_string: &String) {
//     println!("{some_string}");
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }
// fn makes_copy(some_integer: i32) {
//     println!("{some_integer}");
// }

// fn danglin_ref() {}

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     return &s;
// }
