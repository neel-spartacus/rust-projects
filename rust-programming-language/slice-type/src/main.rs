/*fn main() {
    let mut s = String::from("hello world");
    let word = first_word_optimized(&s);
    s.clear();
    println!("{}",word);
   // s.clear();
}*/


fn get_first(strs: &mut (String, String)) -> &mut String {
    &mut strs.0
}
fn get_second(strs: &mut (String, String)) -> &mut String {
    &mut strs.1
}
fn transfer_string(strs: &mut (String, String)) {
    let fst = get_first(strs);
    let snd = get_second(strs);
    fst.push_str(snd);
    snd.clear();
}
fn main() {
    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
}

/*fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word_optimized(&my_string[0..6]);
    let word = first_word_optimized(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word_optimized(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word_optimized(&my_string_literal[0..6]);
    let word = first_word_optimized(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_optimized(my_string_literal);
}*/

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


fn first_word_optimized(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
