// fn main() {
//     let mut x = 50; // mutable variable
//     println!("the value of x is : {}", x);
//     x = 60;
//     println!("the second value of x is : {}", x);
// }

// shadowing is the process where we can assign the value to certain variable and change again its data type

fn main() {
    let x = 50; 
    println!("the value of x is : {}", x);
    let x = 50.5;
    println!("the second value of x is : {}", x);
}

