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
            let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");
            let b_element = b[index];
            let a_element = a[index];
            println!("a[{}]: {},b[{}]: {}", index, index, a_element, b_element);
        }
}
