//TASK 2
fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        // Your implementation here
        tracker += 1;
        println!("{}",tracker);
    };

    update();
    update();
}

//TASK 3
fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

fn main(){
    //TASK 1
    let operation = |a: i32, b: i32| {
        a*b
    };

    println!("Result: {}", operation(10, 5));

    //TASK 2
    track_changes();

    //TASK 3
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        // Implement: multiply each number by 2
        x*2
    });

    let replaced = process_vector(numbers, |mut x| {
        // Implement: if number > 2, replace with 0, else keep number
        if x > 2{
            x = 0;
        }
        x
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}