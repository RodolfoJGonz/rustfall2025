
fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    let mut answer = String::from("");
    answer.push_str(s1);
    answer.push_str(s2);
    return answer
}

fn clone_and_modify(s: &String) -> String{
    // Your code here
    let mut cloned_str = s.clone();
    cloned_str.push_str("World!");
    return cloned_str
}

fn sum(total: &mut i32, low: i32, high: i32){
    // Write your code here!
    *total = 0;
    for i in low..=high {
        *total += i;
    }
}

fn main() {
    println!("Problem 1");
    //Problem 1
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"

        
    println!("\nProblem 2");
    //Problem 2
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"


    println!("\nProblem 3");
    //Problem 3
    let low: i32 = 0;
    let high: i32 = 100;
    let mut ans: i32 = 0;
    sum(&mut ans, low, high);
    println!("Sum: {}", ans);
}

