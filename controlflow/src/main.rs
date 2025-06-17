
//If Else[If Expression][Else Expression]
#![allow(warnings)]
fn main() {
    let age:u16=18;
    if age>=18{
        println!("You can drive a car!");
    }else{
        println!("YOu are not eligible to drive a car!");
    }

    //Using if in a let statement
    let condition:bool=false;
    let number=if condition {5} else {6};
    println!("Number is: {}",number);
}
