#![allow(unused)]
/*fn main() {
    /*let guess:u32 = "42".parse().expect("Not a number!");
    println!("{guess}");*/
       /* let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
        println!("{heart_eyed_cat}")*/


                let tup = (500, 6.4, 1);

                let (x, y, z) = tup;

                println!("The value of y is: {y}");


}*/

/*fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("five_hundred:{}, six_point_four:{}, one:{}",five_hundred,six_point_four,one);
}*/


use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
