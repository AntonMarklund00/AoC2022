fn main(){
    let input = include_str!("./input.txt");
    let split = input.split('\n');

    let mut list:Vec<i32> = Vec::new();
    let mut current = 0;
    for i in split {
        if i == ""{
            list.push(current);
            current = 0;
        }else{
            current += i.parse::<i32>().unwrap();
        }
    }
    list.sort_unstable();
    println!("{}", list.into_iter().rev().take(3).sum::<i32>());
}
