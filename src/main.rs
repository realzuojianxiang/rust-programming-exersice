fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    println!("{}, world!", s1);

    let s2 = s1;
    println!("{}, world!", s2);






}



