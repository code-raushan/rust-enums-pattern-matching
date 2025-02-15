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

    let _my_ip_v4 = IpAddrKind2::V4(String::from("192.0.0.1"));
    let _my_ip_v6 = IpAddrKind2::V6(String::from("1ff2:0370:7334:2c34"));
}
