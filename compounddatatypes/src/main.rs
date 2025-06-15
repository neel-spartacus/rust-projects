fn main() {
    //Compound Data Types
    //arrays,tuples,slices and strings (slice string)

    let numbers:[i32;5]=[1,2,3,4,5];
    println!("Numbers Array: {:?}",numbers);

    let fruits:[&str;3]=["Apple","Banana","Orange"];
    println!("Fruits array: {:?}",fruits);

    //Tuples
    let human:(&str,i32,bool)=("Alice",30,false);
    println!("Human Tuple: {:?}",human);

    let my_mix_tuple=("Alice",30,false,[1,2,3,4,5]);
    println!("Mix tuple: {:?}",my_mix_tuple);

    //Slices
    let number_slices:&[i32]=&[1,2,3,4,5];
    println!(" Number slices : {:?}",number_slices);

    //String vs String Slices
    //String[growable,mutable,owned string type]
    let mut stone_cold:String=String::from("Hell,");
    println!("Stone cold says {}",stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold now says {}",stone_cold);

    //&str(String slice is a reference)
    let string:String=String::from("Hello,World!");
    let slice:&str=&string;
    let partial_slice:&str=&string[0..5];
    println!("Slice value: {}",slice);
    println!("Partial Slice value: {}",partial_slice);
}
