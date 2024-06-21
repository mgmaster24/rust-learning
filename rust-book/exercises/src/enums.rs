#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub fn exercise() {
    let v4 = IpAddrKind::V4;
    route(v4);
    let v6 = IpAddrKind::V6;
    route(v6);

    let home = IpAddr::V4(127, 0, 0, 1);
    println!("IP Address: {:?}", home);

    let loopback = IpAddr::V6(String::from("::1"));
    println!("IP Address: {:?}", loopback);

    println!("Value of coin {}", value_in_cents(Coin::Penny));
    println!("Value of coin {}", value_in_cents(Coin::Nickel));
    println!("Value of coin {}", value_in_cents(Coin::Dime));
    println!("Value of coin {}", value_in_cents(Coin::Quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    print_value(five);
    print_value(six);
    print_value(none);
}

fn plus_one(lhs: Option<i32>) -> Option<i32> {
    match lhs {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_value(opt: Option<i32>) {
    if let Some(val) = opt {
        println!("Value: {}", val);
    } else {
        println!("No Value")
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing to {:?}", ip_kind);
}
