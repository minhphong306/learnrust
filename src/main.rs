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

fn main() {
    // greet_world();
    // penguin();

    variables_scalar();
}
