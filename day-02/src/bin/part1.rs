fn main(){
    let input = include_str!("./input.txt");
    let mut score = 0;
    for line in input.lines() {
        let mut split = line.split(" ");
        let op = split.next().unwrap();
        let me = split.next().unwrap();
        score += match (op, me){
            ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,   
            _ => 3
        };

        score += match me {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0 
        }
    }
    println!("{}", score);
}
