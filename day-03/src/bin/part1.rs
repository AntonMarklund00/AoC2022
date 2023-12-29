fn main(){
    let input = include_str!("./input.txt");
    let mut sum = 0;
    for line in input.lines() {
        let split = line.split_at(line.len()/2);
        let mut common: Vec<&str> = Vec::new();
        for item in split.0.split(""){
            if split.1.contains(&item) && !common.contains(&item){
                common.push(item);
            }
        }

        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
        for item in common{
            if item != "" {
                sum += alphabet.iter().position(|&r| r.to_string() == item).unwrap()+1;
            }
        }
    }

    println!("{}", sum);
}
