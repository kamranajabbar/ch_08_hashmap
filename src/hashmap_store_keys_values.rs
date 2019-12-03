use std::collections::HashMap;

pub fn run() {
    let mut scores = HashMap::new();

    scores.insert(String::from("KK"), 11);
    scores.insert(String::from("IU"), 10);
    println!("Print hashmap : {:?}", scores);

    let team = vec![String::from("PK"), String::from("NZ")];
    let score = vec![101, 113];
    let scoress: HashMap<_, _> = team.iter().zip(score.iter()).collect();
    println!("Print hashmap via vector : {:?}", scoress);
}