


enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

// 可以附加不同类型的值
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn test() {
    let f = IpAddrKind::V4;
    let s = IpAddrKind::V6;
    let home = IpAddr::V4(String::from("127.0.0.1")); // 可以附加值
    let loopback = IpAddr::V6(String::from("::1"));
}




