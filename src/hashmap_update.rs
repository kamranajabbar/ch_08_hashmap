use std::collections::HashMap;

pub fn run() {
    let mut scores = HashMap::new();

    // Replacing a value stored with a particular key
    scores.insert(String::from("KK"), 11);
    scores.insert(String::from("KK"), 22);
    println!("Hashmap after replace {:?}", scores);

    // Only Inserting a Value If the Key Has No Value
    scores.insert(String::from("KK"), 33);
    scores.entry(String::from("KK")).or_insert(44);
    scores.entry(String::from("IU")).or_insert(55);
    println!("Hashmap after insert if key has no value {:?}", scores);

    // Updating a Value Based on the Old Value
    let mut map = HashMap::new();
    let text = "hello world wonderful world";

    for word in text.split_whitespace()
    {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    } 
    
    println!("{:?}", map);
}