// Where structs give you a way of grouping 
// together related fields and data, 
// enums give you a way of saying a value 
// is one of a possible set of values

// enum IpAddrKind {
//   V4,
//   V6,
// }
//
// let four = IpAddrKind::V4;
// let six = IpAddrKind::V6;
// 
// struct IpAddr {
//   kind: IpAddrKind,
//   address: String,
// }
//
// let home = IpAddr {
//   kind: IpAddrKind::V4,
//   address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
//   kind: IpAddrKind::V6,
//   address: String::from("::1"),
// }

// More Concise of above:
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}
// There’s another advantage to using an enum rather than a struct: 
// each variant can have different types and amounts of associated data.

fn main() {
  let home = IpAddr::V4(127,0,0,1);
  let loopback = IpAddr::V6(String::from("::1"));
}
