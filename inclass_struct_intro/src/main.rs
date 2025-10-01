// Assignment 1
// Declaring Struct
struct Student{
    name: String,
    major: String,
}

// Assignment 2 struct
struct ParkingSystem{
    big:i32,
    medium:i32,
    small:i32,
}

// Assignment 1 Implementing Struct
impl Student{
    // Constructor
    fn new(n: String, m: String) -> Student{
        Student{
            name: n,
            major: m,
        }
    }

    // You have to borrow_to_read because if not, you cannot use multiple times.
    fn get_major(&self) -> &String{
        return &self.major
    }

    fn set_major(&mut self, new_major:String){
        self.major = new_major;
    }
}

// Assignment 2 Parking System implementation
impl ParkingSystem{
    fn new(big:i32,medium:i32,small:i32) -> Self{
        ParkingSystem{
            big: big,
            medium: medium,
            small: small,
        }
    }
    
    fn add_car(&mut self, car_type: i32) -> bool{
        let available: bool = if car_type == 1 {
            if self.big > 0 {
                self.big -= 1;
                true
            }
            else{return false}
        }else if car_type == 2 {
            if self.medium > 0 {
                self.medium -= 1;
                true
            }
            else{false}
        }else{
            if self.small > 0 {
                self.small -= 1;
                true
            }
            else{false}
        };
        return available
    }
}

fn main() {
    //Assignment 1
    println!("Assignment 1");
    let mut stud1 = Student::new("Rodolfo".to_string(), "Comp Sci".to_string());
    println!("{}: {}",stud1.name, stud1.get_major());
    println!("{}: {}",stud1.name, stud1.get_major());
    println!("{}: {}",stud1.name, stud1.get_major());
    stud1.set_major("Business".to_string());
    println!("{}: {}",stud1.name, stud1.get_major());
    println!("{}: {}",stud1.name, stud1.get_major());
    println!("{}: {}",stud1.name, stud1.get_major());

    //Asignment 2: 1603. Design Parking System Leetcode
    println!("\nAssignment 2");
    let mut parking_system:ParkingSystem = ParkingSystem::new(1,1,0);
    println!("{}",parking_system.add_car(1)); // return true because there is 1 available slot for a big car
    println!("{}",parking_system.add_car(2));  // return true because there is 1 available slot for a medium car
    println!("{}",parking_system.add_car(3));  // return false because there is no available slot for a small car
    println!("{}",parking_system.add_car(1));  // return false because there is no available slot for a big car. It is already occupied.
    

}
