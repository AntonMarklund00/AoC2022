fn main(){
    let input: Vec<&str> = include_str!("./input.txt").split("\n").collect();
    let data: Vec<Vec<&str>> = input.chunks(3).map(|s| s.to_vec()).collect();

    let mut common: Vec<char> = Vec::new();

    for i in data {
        let mut chunk_common: Vec<char> = Vec::new();
        for j in i.get(0).unwrap().chars() {
            if i.get(1).unwrap().contains(j) && i.get(2).unwrap().contains(j) && !chunk_common.contains(&j) {
                chunk_common.push(j);
            }
        }
        common.append(&mut chunk_common);

    }
    let mut sum = 0;
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
    for item in common{
        sum += alphabet.iter().position(|&r| r == item).unwrap()+1;
    }

    println!("{}", sum);
}
