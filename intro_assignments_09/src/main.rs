
use std::io;

//Assignment 1
const FREEZE: f64 = 32.0; //32 F

fn fahrenheit_to_celsius(f: f64) -> f64 {
    return (f-FREEZE)*(5.0/9.0)
}


fn celsius_to_fahrenheit(c: f64) -> f64 {
    return c*(9.0/5.0)+FREEZE
}


//Assignment 2
fn is_even(n:i32) -> bool {
    if n % 2 == 0 {
        return true
    }
    else{
        return false
    }
}

//Assignment 3
fn check_guess(guess: i32, secret: i32) -> i32{
    let correct: i32 = if guess == secret{
        0
    } else if guess > secret{
        1
    } else{-1};
    return correct
}


fn main() {
    println!("Assignment 1");
    //Assignment 1
    let mut temp: f64 = 32.0;
    println!("\nTemperature(F): {}",temp);

    temp = fahrenheit_to_celsius(temp);
    println!("Temperature(C): {}",temp);

    for _i in 0..=5{

        temp = celsius_to_fahrenheit(temp);
        temp += 1.0;
        println!("\nTemperature(F): {}",temp);

        temp = fahrenheit_to_celsius(temp);
        println!("Temperature(C): {}",temp);

    }

    println!("\nAssignment 2");
    //Assingment 2
    let nums = [12,3,5,6,8,15,2,1,11,35];
    for num in nums.iter(){

        let div_by = if num % 3 == 0 && num % 5 == 0{
            "FizzBuzz"
        } else if num % 3 == 0 {
            "Fizz"
        }else if num % 5 == 0{
            "Buzz"
        }else if is_even(*num){
            "Even"
        }else {"Odd"};
        println!("Number {}: {}", num, div_by);
    }

    let mut itr: usize = 0;
    let mut sum: i32 = 0;
    while itr < nums.len(){
        sum += nums[itr];
        itr += 1;
    };
    println!("Sum: {}", sum);

    itr = 0;
    let mut x: i32 = 0;
    let max: i32 = loop {
        if itr == nums.len(){
            break x
        };
        if nums[itr] > x{
            x = nums[itr];
        };
        itr += 1;
    };
    println!("Max: {}", max);


    println!("\nAssignment 3");
    //Assingment 3
    let sec_num = 43;
    //let mut input = String::new();
    //let mut input_num: i32 = 0;
    let mut counter: i32 = 0;
    loop {
        let mut input = String::new();
        //Looked this line up
        println!("\nGuess a number: ");
        io::stdin().read_line(&mut input).expect("Invalid String.").to_string();
        let input_num: i32 = input.trim().parse().expect("Could not Parse.");

        let guess = check_guess(input_num, sec_num);
        if guess == 0{
            counter += 1;
            println!("Guess was correct!");
            break
        }else if guess == 1{
            counter += 1;
            println!("Guess was too high.");
        }else{
            counter += 1;
            println!("Guess was too low.");
        };
    };
        println!("Total Guesses: {}", counter);
}
