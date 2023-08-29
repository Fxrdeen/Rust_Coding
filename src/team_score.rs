use std::io;
use std::collections::HashMap;
pub fn team_map(){
    let mut scores: HashMap<String, i32> = HashMap::new();
    let mut no = String::new();
    println!("Enter the number of teams:");
    io::stdin().read_line(&mut no).expect("Bruh");
    let _no: i32 = match no.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Bro dont you understand what a number is? Program is exiting");
            0
        }
    };
    if _no == 0{
        return;
    }
    for i in 0.._no {
        println!("Enter the name of the {} team: ",i+1);
        let mut team: String = String::new();
        io::stdin().read_line(&mut team).expect("Bruh");
        let team:String = team.trim().parse().expect("Bruh");
        println!("Enter the score of team {}",i+1);
        let mut score = String::new();
        io::stdin().read_line(&mut score).expect("Bruh");
        let score = match score.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("BRO Its says enter the score, so it has to be an integer!!. Assigning score as zero to the team");
                0
            } 
        };
        scores.insert(team, score);
    }
    println!("The team with thier corresponding score is:");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let mut a = String::new();
    println!("Enter the team to be checked");
    io::stdin().read_line(&mut a).expect("Bruh");
    let a:String = a.trim().parse().expect("Bruh");
    let res = check_score(&scores, &a);
    if res!=-1{
        println!("The value of the score of team {a} is {res}");
    } else{
        println!("The team does not exist in this map");
    }
}
pub fn check_score(map: &HashMap<String,i32>, key:&String) -> i32 {
    for i in map {
        if key == i.0{
            return *i.1;
        }
    }
    -1
}