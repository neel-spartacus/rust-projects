
/*fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    let full_name=stringify_name_with_title(&name);
    println!("{}", first);
    println!("{}", full_name);
}
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}*/

/*fn main(){
    /*let mut name =String::from("Neel");
    println!("{}",append_degree(&mut name));*/

   /* let mut vec:Vec<String>=vec!["Whale".parse().unwrap(), "is".parse().unwrap(), "the".parse().unwrap()];
    let str=[String::from("biggest"),String::from("animal")];
    add_big_strings(&mut vec,&str);
    println!("{:?}", vec);*/

    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];
    println!("{:?}", v);
    println!("{n_ref}");

}*/

/*fn main() {
    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean")
    );
    let first = &name.0;
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}*/

fn get_first(name: &(String, String)) -> &String {
    &name.0
}

/*fn main() {
    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean")
    );
    let first = get_first(&name);
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}*/

fn main() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    *x += 1;
    println!("{a:?}");
}

/*fn main() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    let y = &a[2];
    *x += *y;
}*/




/*fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let n = &mut v[i];
    *n = v[i - 1];
}
fn main() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 1);
}*/

fn append_degree(name:&mut String)->&String{
    //let mut name=name.clone();
    name.push_str(",Phd");
    name

}

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: usize =
        dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest {
            dst.push(s.clone());
        }
    }
}

// ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."