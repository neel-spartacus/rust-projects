fn main() {
    println!("Hello, world!");
    human_id("Neel",37,167.5);
    //Expressions
    let x:i32={
        let price:i32=50;
        let quantity:i32=5;
        price*quantity
    };

    println!("Result is {}",x);
    let y:i32=add(4,6);
    println!("Value of y is {}",y);
    println!("Value from function 'add' is:  {}.",add(4,5));

    let weight:f64=56.2;
    let height:f64=1.67;
    println!("Your BMI value is {:.2}.",calculate_bmi(weight,height));
}

fn add(a:i32,b:i32)->i32{
    a+b
}

fn human_id(name:&str,age:u32,height:f32){
    println!("My name is {}.I am {} years old and my height is {} cm.",name,age,height);
}

fn calculate_bmi(weight_kg:f64,height_m:f64)->f64{
    weight_kg/(height_m*height_m)
}

//Expressions and Statements
//Expression: Anything that returns a value.
//Example: 5,true&false,if condition {value1} else {value2}.
//Statement: Anything that does not return a value.
//Almost all statements in Rust end with ;