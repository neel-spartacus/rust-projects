fn main() {
    let mut _x=5;
    const  Y: i32 =10;
    //note: each usage of a `const` item creates a new temporary
    //the mutable reference will refer to this temporary, not the original `const` item
    let z=&mut Y;
    *z+=10;
    println!("Value of _x is: {}",_x);
    println!("Value of Y is: {}",Y);
    println!("Value of z is: {}",z);
    println!("Value of Y is: {}",Y);
    println!("Value of Z is: {}",Z);

}

//Declare const in global scope with a type annotation
const Z:i32=25;
