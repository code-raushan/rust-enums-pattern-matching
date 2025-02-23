// enum syntax
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// #[derive(Debug)]
enum IpAddrKind2 {
    V4(String),
    V6(String),
}

fn main() {
    let _ipv4 = IpAddrKind::V4;
    let _ipv6 = IpAddrKind::V6;

    let loopback = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("{}", loopback.address);
    println!("{:?}", loopback.kind);

    // enum with values
    let my_ip_v4 = IpAddrKind2::V4(String::from("192.0.0.1"));
    let _my_ip_v6 = IpAddrKind2::V6(String::from("1ff2:0370:7334:2c34"));

    match my_ip_v4 {
        IpAddrKind2::V4(ip) => println!("IPv4 address: {}", ip),
        IpAddrKind2::V6(ip) => println!("IPv6 address: {}", ip), // We don't need to handle V6 case but match needs to be exhaustive
    }

    // enum _Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number: Option<i32> = Some(6);
    let x = 5;

    let sum = x + some_number.unwrap_or(0);
    println!("{}", sum);

    fn _plus_one(x: Option<u32>) -> Option<u32> {
        match x {
            None => None,
            Some(i) => Some(i+1)
        }
    }

    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("three");
    }
}
