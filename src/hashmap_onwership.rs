use std::collections::HashMap;

pub fn run() {
    let mut map = HashMap::new();

    let team1 = String::from("KK");
    let team2 = String::from("IU");

    map.insert(team1, team2);
    println!("Print hashmap : {:?}", map);
    //println!("{}", team1); // team1 and team2 are invalid at this point due to value moved into the hashmap.
}