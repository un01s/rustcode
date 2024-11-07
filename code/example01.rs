fn add(i: i32, j: i32) -> i32 {
    return i + j;
}

fn low_level() {
    let penguin_data = "\
    common name,length (cm)
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

    let fields: Vec<_> = record
        .split(',')
        .map(|field| field.trim())
        .collect();
    if cfg!(debug_assertions) {
        eprintln!("debug: {:?} -> {:?}",
            record, fields);
    }

    let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好!";
    let english = "hello world!";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
    low_level();
    let a = 10;
    let b: i32 = 20;
    println!("{}", add(a, b));
}
