
//Shadowing
//Shadowing is not the same as marking a variable as mutable.
fn main() {
    let x=5;
    let y=x;
    {
        let x=x*2;
        println!("The value of x inside the scope is : {}",x);
    }

    println!("The value of x is: {}",x);
}
