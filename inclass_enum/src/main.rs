#[derive(PartialEq,Debug)]
enum Fruit{
    Apple(String),
    Banana(String),
    Tomato(String),
}

struct Inventory{
    fruit:Vec<Fruit>,
}

impl Inventory{
    fn available_fruits(&self){
        for f in &self.fruit{
            // Doing this match statement instead of the for loop we used in class
            // because the for loop printed out the joke for a second time.
            match f {
               Fruit::Apple(_) => print!("Apple: "),
               Fruit::Banana(_) => print!("Banana: "),
               Fruit::Tomato(_) => print!("Tomato: "),
            }  
            Self::tell_me_joke(f);
        }
    }

    fn tell_me_joke(fruit:&Fruit){
        match fruit{
            Fruit::Apple(msg) => println!("{}",msg),
            Fruit::Banana(msg) => println!("{}",msg),
            Fruit::Tomato(msg) => println!("{}",msg),
        }
    }
}

fn main() {
    let a = "Why did the apple stop in the middle of the road? Because it ran out of juice!".to_string();
    let b = "Why did the banana go to the doctor? It wasn’t peeling very well!".to_string();
    let t = "Why did the tomato turn red? Because it saw the salad dressing!".to_string();
    let fruits = vec![Fruit::Banana(b), Fruit::Apple(a), Fruit::Tomato(t)];

    let grocery_store = Inventory {
        fruit:fruits,
    };

    grocery_store.available_fruits();
}

