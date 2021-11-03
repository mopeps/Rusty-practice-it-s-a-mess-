enum IpAddrKind {
    v4,
    v6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let homeKind = IpAddr {
    kind: IpAddrKind::v4,
    address: String::from("127.0.0.1"),
};


let loopbackKind = IpAddr {
    kind: IpAddrKind::v6,
    address: String::from("::1"),
};

//fn route(ip_kind: IpAddrKind) {}

/*
enum IpAddr {
    v4(String),
    v6(String),
}

let home = IpAddr::v4(String::from("127.0.0.1"));
let loopback = IpAddr::v6(String::from("::1"));

*/

enum IpAddr {
    v4(u8,u8,u8,u8),
    v6(String),
}

let home = IpAddr::v4(127, 0, 0, 1)l

let loopback = IpAddr::v6(String::from("::1"));



fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

}
