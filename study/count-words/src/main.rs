use std::collections::HashMap;

fn main() {
    let mut words: HashMap<String, u32> = HashMap::new();
    let mut input = String::new();

    loop {
        input.clear();
        if 0 == std::io::stdin().read_line(&mut input).unwrap() {
            break;
        }

        for word in input.split_whitespace().map(String::from) {
            let count = words.entry(word).or_default();
            *count += 1;
        }
    }
    
    println!("{:?}", words);
}
