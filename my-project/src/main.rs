fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mean = numbers.iter().sum::<i32>() as f32 / numbers.len() as f32;
    let median = numbers[numbers.len() / 2];

    println!("mean: {}, median: {}", mean, median);
}