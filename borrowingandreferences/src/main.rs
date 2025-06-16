fn main() {
    let mut _x:i32=5;
    let _y:&mut i32 =&mut _x;
    *_y+=1;
    *_y-=2;

    // cannot borrow `_x` as immutable because it is also borrowed as mutable
    // To fix this error, ensure that you don't have any other references to the variable before trying to access it with a different mutability as in below(correct version):
    // println!("Value of _x: {}",_x);
    // println!("Value of _y: {}",_y);

    //correct version
    println!("Value of _y: {}",_y);
    println!("Value of _x: {}",_x);
}
