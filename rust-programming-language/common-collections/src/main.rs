/*fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: i32 = v[2];
    let fourth: i32 = v[3];
    println!("The third element is {third}");
    println!("The fourth element is {fourth}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}*/


/*fn main() {
    let mut v = Vec::new();
    let s = String::from("Hello ");
    v.push(s);
    v[0].push_str("world");
    println!("original: {}", s);
    println!("new: {}", v[0]);
}
*/

/*fn main() {
    let v = vec![String::from("Hello ")];
    let mut s = v[0];
    s.push_str("world");
    println!("{s}");
}*/

/*fn main() {
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        v.push(*i);
    }
    println!("{} {} {}", v[3], v[4], v[5]);
}*/

/*fn main(){
    let hello = "Здравствуйте";
    let answer = &hello[0..2];
    println!("{}", answer);
}*/

/*fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name.clone(), field_value.clone());
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("{} : {} ", field_name, field_value);
}*/

/*fn main(){

use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
let count = map.entry(word).or_insert(0);
*count += 1;
}

println!("{map:?}");
}*/

use std::collections::HashMap;
fn main() {
    let mut h: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c) in "hello!".chars().enumerate() {
        h.entry(c).or_insert(Vec::new()).push(i);
    }
    let mut sum = 0;
    for i in h.get(&'l').unwrap() {
        sum += *i;
    }
    println!("{}", sum);
}