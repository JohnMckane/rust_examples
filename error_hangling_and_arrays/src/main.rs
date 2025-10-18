use std::io;
fn main() {
    let a = [1, 2, 3, 4 ,5];
    let b = [2; 5];
    loop {
        println!("Enter index");
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
        let index = match index
            .trim()
            .parse() {
                Ok(index) => (index, true),
                Err(_error) => (0, false),
            };
            if !index.1 {
                println!("Did not parse int");
                continue;
            }
            let index = index.0;
            if index < a.len() && index < a.len(){
            let b_element = b[index];
            let a_element = a[index];
            println!("a[{}]: {},b[{}]: {}", index, index, a_element, b_element);
            }else {
                println!("out of bounds");
        }
    }
}
