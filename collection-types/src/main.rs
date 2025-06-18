
//Collection types
//Vectors - UTF8 - Hashmaps

fn main() {
    let mut _v:Vec<i32>=Vec::new();
    let _the_vec:Vec<i32>=vec![1,2,3];
    _v.push(2);
    _v.push(3);
    _v.push(4);

    let _third_element:&i32=&_v[2];
    println!("{:?}",_v);
    println!("The third element is: {}",_third_element);

    let _third:Option<&i32>=_v.get(4);
    match _third {
        Some(e)=>println!("The element is: {}",e),
        None=>println!("No given element present in the vector.")
    }
}
