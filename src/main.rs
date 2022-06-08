// From book
fn greet_world() {
    println!("Hello, world!");

    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];

    for region in regions.iter() {
        println!("{}", &region); // Dấu `&` mượn region để read only access
    }
}

fn penguin() {
    let penguin_data = "\
    common name, length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];

        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}

// From rush crash course for solana (https://www.youtube.com/watch?v=-AAtfPHEMbA&list=PL53JxaGwWUqCr3xm4qvqbgpJ4Xbs4lCs7&index=2)

// variables - scalar
fn variables_scalar() {
    let unsigned: u8 = 10;

    let signed: i8 = -10;

    let float: f32 = 1.2;

    println!("unsign: {}, sign: {}, float: {}", unsigned, signed, float);

    let letter = "c1232";
    let emoji = "\u{1F600}";

    println!("letter: {}, emoji: {}", letter, emoji);

    let is_true: bool = true;

    println!("isTrue: {}", is_true);
}

fn array() {
    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5];

    println!("index: {}, length: {}", arr[0], other_arr.len());

    // Print structure of array and other objects
    println!("{:?}", arr);
    println!("{:?}", other_arr);
}

fn tuple() {
    let tuple: (u8, bool, f32) = (5, true, 2.1);
    let tuple2 = (3, 5);

    let (a, b, c) = tuple;

    println!("First: {}, second: {}, third: {}", a, b, c);
}

fn function() {
    print!("{}", is_even(2))
}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 // return bool
}

fn mutability() {
    let mut num = 5;
    num = 3;
    println!("{}", num);
}

fn string() {
    let str: &str = "hello world";
    let mut string: String = String::from("Hello World");

    let slice = &string[.. 6];
    slice.len();

    string.push('1');
    string.push_str("! Bob");
    string = string.replace("Hello", "Bye");
    println!("{}", string);
}

fn array_and_slice() {
    let arr = [0, 1, 2, 3];
    let slice = &arr[1 .. 3];

    borrowing_slice(arr, slice);
}

fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}

fn if_condition() {
    let n = 3;
    if n > 0 {
        println!("greater than 0");
    } else if n < 0 {
        println!("less than 0");
    } else {
        println!("equal zero");
    }
}

fn for_loop() {
    for i in 0..6 {
        println!("{}", i);
    }
}

fn while_loop() {
    let mut i = 0;
    while i < 4 {
        println!("{}", i);
        i += 1;
        
        if i == 3 {
            println!("exit");
            break;
        }
    }
}

fn match_statement() {
    let i = 5;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1, 2"),
        3..=4 => println!("3, 4"),
        _ => println!("default")
    }
}

struct Bird {
    name: String,
    attack: u64
}

impl Bird {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}

fn structs() {
    let name = String::from("Bird");
    let bird = Bird { name: name, attack: 100};

    bird.print_name();
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }

    fn is_animal(&self) -> bool {
        false
    }
}

fn trail() {

}

#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C {x: i32, y: i32}
}

// fn enum() {
//     let a: MyEnum = MyEnum::A;
//     let b: MyEnum = MyEnum::B(5);
//     let c: MyEnum = MyEnum::C{x: 10, y: 20};
// }


// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn test_lifetime() {
//     println!("Hello hihi");
//     let string1 = String::from("abcd");
//     let string2 = "xyz";
//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

fn longest_v1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test_lifetime_v1() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_v1(&string1, string2);
    println!("The longest string is {}", result);
}

fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn main() {
    // greet_world();
    // penguin();

    // variables_scalar();~
    // array();
    // tuple();
    // function();
    // mutability();
    // array_and_slice();
    // string();
    // if_condition();
    // for_loop();
    // while_loop();
    // match_statement();
    // structs();
    // test_lifetime();
    // test_lifetime_v1();
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b);

    println!("Result is: {}", res);
}