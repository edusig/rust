enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("Hello, world!");
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;
    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
