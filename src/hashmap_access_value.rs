use std::collections::HashMap;

pub fn run() {
    let mut scores = HashMap::new();

    let team1 = String::from("KK");
    let team2 = String::from("IU");

    let score1 = 11;
    let score2 = " -10 ";
    let score_2:i32 = score2.trim().parse().unwrap();

    scores.insert(team1, score1);
    scores.insert(team2, score_2);

    // get value from key
    //let look_team_name = String::from("KK");
    //let found = scores.get(&look_team_name);
    //println!("Result found {:?}", found);

    // iterate over each key/value pair in a hash map
    for (key, value) in &scores { 
        println!("{}: {}", key, value);
    }
}