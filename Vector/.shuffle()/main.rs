use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut nums: Vec<i32> = Vec::new();
    
    nums.push(1);
    nums.push(2);
    nums.push(3);
    nums.push(10);
    
    nums.sort();
    nums.shuffle(&mut rand::rng());

    println!("{:?}", nums);
}
