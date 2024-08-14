pub fn exercise() {
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("\tThe number {x} is even"),
        Some(x) => println!("\tThe number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("\tGot 50"),
        Some(n) if n == y => println!("\tMatched, n = {n}"),
        _ => println!("\tDefault case, x = {x:?}"),
    }

    println!("\tat the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("\tOr operator with match guard - yes"),
        _ => println!("\tOr operator with match guard - no"),
    }
}
