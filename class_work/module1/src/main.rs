 
const FREEZING:f64 = 32.0;

fn fahrenheit_to_celsius(f:f64) -> f64{
    return (f-FREEZING)/(9.0/5.0)
}

fn celsius_to_fahrenheit(c:f64) -> f64{
    return (c*(5.0/9.0))+FREEZING
}

fn main() {
    let mut temp:f64 = 32.0;
    println!("{}: {}",temp,fahrenheit_to_celsius(temp));

    println!("Starting loop...");
    for n in 0..5{
        temp += 1.0;
        println!("{}: {}",temp,fahrenheit_to_celsius(temp));
    }

}
