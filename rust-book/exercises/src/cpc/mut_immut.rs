pub fn exercise() {
    // mutable and immutable references
    let mut s = String::from("immutable ref");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    r3.push_str(" and a mutable one.");
    println!("{}", r3);
}
