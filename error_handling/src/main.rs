
//Error handling techniques[2 approaches]

    //Approach 1
  /*  enum Option<T>{//Define the generic Option type
        Some(T),//Represents a value
        None //Represents no value
    }*/

    //Approach 2
   /* enum Result<T,E>{ //Define the generic Result type
        Ok(T), //Represents a value
        Err(E) //Represents an error
    }*/


   fn divide(numerator:f64,denominator:f64)->Option<f64>{
       if denominator==0.0{
           None
       }else{
           Some(numerator/denominator)
       }
   }
fn divideResult(numerator:f64,denominator:f64)->Result<f64, String>{
    if denominator==0.0{
        Err("Cannot divide by Zero!".to_string())
    }else{
        Ok(numerator/denominator)
    }
}
fn main() {

    let result=divide(10.0,0.0);
    match result {
        Some(x)=>println!("Result is: {}",x),
        None=>println!("Cannot divide by zero!!")
    }

    let result2=divideResult(10.0,0.0);
    match result2 {
        Ok(x)=>println!("The result is: {}",x),
        Err(e)=>println!("Error:{}",e)
    }

}
