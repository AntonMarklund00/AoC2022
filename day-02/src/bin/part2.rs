fn main(){
    let input = include_str!("./input.txt");
    let mut score = 0;
    for line in input.lines() {
        let mut split = line.split(" ");
        let op = split.next().unwrap();
        let me = get_correct_hand(op, split.next().unwrap());

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

fn get_correct_hand<'a>(op: &'a str, me: &'a str) -> &'a str{
    
    return match me {
        "Y" => get_tie(op),          
        "X" => get_loss(op),          
        "Z" => get_win(op),          

        _ => panic!() 
    };
}

fn get_tie(op: &str) -> &str{
    return match op {
        "A" => "X",
        "B" => "Y",
        "C" => "Z",
        _ => panic!(),
    };
}

fn get_win(op: &str) -> &str{
    return match op {
        "A" => "Y",
        "B" => "Z",
        "C" => "X",
        _ => panic!(),
    };
}

fn get_loss(op: &str) -> &str{
    return match op {
        "A" => "Z",
        "B" => "X",
        "C" => "Y",
        _ => panic!(),
    };
}
