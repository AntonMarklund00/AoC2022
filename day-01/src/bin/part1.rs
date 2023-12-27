fn main(){
    let input = include_str!("./input.txt");
    let split = input.split('\n');

    let mut sum = 0;
    let mut current = 0;
    for i in split {
        if i == ""{
            if current > sum{
                sum = current;
            }
            current = 0;
        }else{
            current += i.parse::<i32>().unwrap();
        }
    }
    println!("{}", sum);
}
