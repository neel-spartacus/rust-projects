
//Structs are used to name and package related values similar to tuples.

fn main() {
    // tuple
    let rect=(200,100);
    struct Book {
        title:String,
        author:String,
        pages:u32,
        available:bool
    }
    struct User{
        active:bool,
        username:String,
        email:String,
        sign_in_count:u64
    }
    let mut user1:User= User{
        active:true,
        username:String::from("Neel"),
        email:String::from("neel@hotmail.com"),
        sign_in_count:1
    };
    user1.email=String::from("neel@awol.com");
    println!("User1 email is: {}",user1.email);
    let user2:User=User{
        email:String::from("neel@yahoo.com"),
        ..user1
    };
    println!("User 2: Name: {},Email: {}, active:{}",user2.username,user2.email,user2.active);

    //Tuple structs
    struct Color(i32,i32,i32);
    let black:Color=Color(0,0,0);
    let white:Color=Color(255,255,255);
}
