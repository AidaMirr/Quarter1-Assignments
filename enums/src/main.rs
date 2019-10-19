// Direction Enum
/*
fn main() {
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

check_direction(Direction::Backward);

fn check_direction(direction: Direction){
    match direction {
        Direction::Forward => { println!("Vehicle is moved Forward."); },
        Direction::Backward => { println!("Vehicle is moved Backward."); },
        Direction::Left => { println!("Vehicle is moved Left."); },
        Direction::Right => { println!("Vehicle is moved Right."); },
    }
}
}
*/

//Option Enum
/*
fn main() {
    let num = Some(10);
    let none = add_one(None);
    let num_plus_one = add_one(num);

fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

println!("{:?}",num_plus_one);
println!("{:?}",none);

}
*/