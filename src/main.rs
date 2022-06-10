use std::io::Read;
use regex::Regex;
fn main() {
    let org_arr = [1,2,3,5,6,8,10,11];
    let sub_arr = [6,8,10];
    println!("Question 1");
    println!("Answer: {}", check_arr(&org_arr, &sub_arr));
    println!("Question 2");
    let content = read_input_file();
    println!("Input your word: ");
    let word = read_from_input();
    let mut regText = "(?i)".to_owned();
    regText.push_str(&word);
    let re = Regex::new("(?i)oF").unwrap();
    let match_count = re.find_iter(&content).count();
    println!("Matched {} times", match_count);
}
fn check_arr(org_arr: &[i32], sub_arr: &[i32]) -> bool{
    if org_arr.len() < sub_arr.len(){
        return false;
    }
    let mut temp : &[i32] = &org_arr[0..];
    loop {
        if temp.starts_with(sub_arr){
            return true;
        }else{
            temp = &temp[1..];
        }
        if temp.len() < sub_arr.len(){
            return false;
        }
    }
}

fn read_input_file() -> String{
    std::fs::read_to_string("src/input.txt").expect("Something went wrong when read input.txt")
}
fn read_from_input() -> String{
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Something went wrong when read input");
    buffer
}