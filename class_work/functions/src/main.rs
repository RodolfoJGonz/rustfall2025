// fn add(x:i32, y:i32) -> i32 {
//     //return x + y; //statement
//     x+y
// }

//Last line of a function is evaluated as an expression and returned
//if no semicolon is added.
//fn add(x:i32, y:i32) -> u8 {(x-y).try_into().unwrap()}

fn fizzbuzz(num:i32) -> String {
    let by3 = num%3 == 0;
    let by5 = num%5 == 0;

    match(by3, by5){
        (true, true) => "FizzBuzz",
        (true, false) => "Fizz",
        (false, true) => "Buzz",
        (false, false) => todo!(),
    }.to_string()
}


fn main() {
    for num in 1..20{
        println!("{}", fizzbuzz(num));
    }
}
