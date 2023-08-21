///
/// 
/// 
/// ######################################################################################################################################################
///  枚举的定义和使用   【枚举成员默认就是公有的】
/// ###################################################################################################################################################### 
/// 
/// 
/// 
/// 
/// enum IpAddrKind {
///     V4,
///     V6,
/// }
///
/// fn main() {
///     let four = IpAddrKind::V4;
///     let six = IpAddrKind::V6;
///
///     route(IpAddrKind::V4);
///     route(IpAddrKind::V6);
/// }
///
/// fn route(ip_kind: IpAddrKind) {}
///
/// 
/// ######################################################################################################################################################
///  有数据的枚举
/// ###################################################################################################################################################### 
/// 
/// 
/// fn main() {
///     enum IpAddr {
///         V4(String),
///         V6(String),
///     }
/// 
///     let home = IpAddr::V4(String::from("127.0.0.1"));
/// 
///     let loopback = IpAddr::V6(String::from("::1"));
/// }
/// 
/// 
/// ######################################################################################################################################################
///  枚举替代结构体
/// ###################################################################################################################################################### 
/// 
/// 每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数.
/// 
/// 也就是说，IpAddr::V6(String) 是一个获取 String 参数并返回 IpAddr 类型实例的函数调用。作为定义枚举的结果，这些构造函数会自动被定义.
/// 
/// 用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据.
/// (结构体的字段是固定的， 而枚举因为 不同的枚举值 <可以看做某类型的结构体> 的中的参数 <可以看做是 结构体的各个字段> 是可以不一样的，所以某个类型的枚举可以由多个枚举值去处理类似多种字段不一样的结构体)
/// 
/// 
/// fn main() {
///     enum IpAddr {
///         V4(u8, u8, u8, u8),
///         V6(String),
///     }
/// 
///     let home = IpAddr::V4(127, 0, 0, 1);
/// 
///     let loopback = IpAddr::V6(String::from("::1"));
/// }
/// 
/// 
/// ######################################################################################################################################################
///  内嵌了多种多样的类型的枚举
/// ###################################################################################################################################################### 
/// 
/// 
/// enum Message {
///     Quit,
///     Move { x: i32, y: i32 },
///     Write(String),
///     ChangeColor(i32, i32, i32),
/// }
/// 
/// fn main() {}
/// 
/// 
/// ######################################################################################################################################################
///  给枚举定义方法
/// ###################################################################################################################################################### 
/// 
/// 
/// fn main() {
///     enum Message {
///         Quit,
///         Move { x: i32, y: i32 },
///         Write(String),
///         ChangeColor(i32, i32, i32),
///     }
/// 
///     impl Message {
///         fn call(&self) {
///             // 在这里定义方法体
///         }
///     }
/// 
///     let m = Message::Write(String::from("hello"));
///     m.call();
/// }
