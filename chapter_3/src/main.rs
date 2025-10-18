fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is {x}");
    println!("Shadow example");
    let x = 9;
    println!("x is {x}");
    {
        let x = 6;
    println!("x is {x} in this scope");
    }
    println!("x is {x} in main");
    println!("Scope to change type");
    let spaces = "     ";
    println!("spaces: {spaces}");
    let spaces = spaces.len();
    println!("spaces: {spaces}");

}
