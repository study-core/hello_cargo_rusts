/// 
///                     【结构体成员默认就是      私有的   】  【 enum 的各个值默认不是 私有的哦 】
/// 
/// 
/// 
///                 结构体的字段也是一个可见性的层次。【字段默认拥有私有的可见性】，也可以加上 pub 修饰语来重载该行为。
/// 
///                 只有从结构体被定义的模块之外访问其字段时，这个可见性才会起作用 (即： 不是 pub 的，外面【不可见】)，其意义是隐藏信息。
/// 
/// ********************************************************************
/// 
/// user1 的任何字段都不可以更改
/// 
/// ********************************************************************
/// 
/// fn main() {
/// 
///     // 只用  let  
/// 
///     let user1 = User {                                              // 注意，此处这样写会报错，原因是 User 未定义
///         active: true, 
///         username: String::from("someusername123"), 
///         email: String::from("someone@example.com"), 
///         sign_in_count: 1, 
///     };
/// }
/// 
/// 
/// ********************************************************************
/// 
/// user1 可以修改自身的 字段
/// 
/// ********************************************************************
/// 
/// 
/// fn main() {
/// 
///     // 用了  let  mut
///     
///     let mut user1 = User {                                          // 注意，此处这样写会报错，原因是 User 未定义
///         active: true, 
///         username: String::from("someusername123"), 
///         email: String::from("someone@example.com"), 
///         sign_in_count: 1, 
///     };
/// 
///     user1.email = String::from("anotheremail@example.com");
/// }
///
/// 
/// 
/// 
/// ********************************************************************
/// 
/// user2 剩余的字段使用 user1 的
/// 
/// ********************************************************************
/// 
/// 
/// 
/// 
/// fn main() {
///     // --snip--
/// 
///     let user2 = User {                                      // 注意，此处这样写会报错，原因是 User 未定义
///         email: String::from("another@example.com"), 
///         ..user1                                             //  user1 的所有权已经被转移到 user2 了哦
///     };
/// 
///  println!("user: {#:?}",  user1);                           // 报错: user1 的所有权已经转移到 user2 了  (user2 使用了 user1 的非基础类型的字段，导致 user1 的所有权被转移，看下一例子就明白)
/// }
/// 
/// ----------------------------------------------------
/// fn main() {
///     // --snip--
/// 
///     let user2 = User {                                      // 注意，此处这样写会报错，原因是 User 未定义
///         email: String::from("another@example.com"), 
///          username: String::from("user2"), 
///         ..user1                                             //  user1 还是可以用哦
///     };
/// 
///  println!("user: {#:?}",  user1);                           // 可以用 user1,  因为 复制给 user2 的只有 active [bool] 和 sign_in_count [u32] (因为基础类型实现了 Copy  trait, 产生 移动所有权)
///  
/// }
/// 
/// 
/// 
/// 
/// 
/// ######################################################################################################################################################
///  元组结构体   (像使用 元组一样使用)
/// ######################################################################################################################################################
/// 
/// 没有具体的字段名, 只有字段的类型.
/// 
/// 【用途】:
/// 
///         当你想给整个元组取一个名字, 并使元组成为与其他元组不同的类型时, 元组结构体是很有用的
/// 
/// 
/// struct Color(i32,  i32,  i32);
/// struct Point(i32,  i32,  i32);
/// 
/// fn main() {
///     let black = Color(0,  0,  0);
///     let origin = Point(0,  0,  0);
/// }
/// 
/// 
/// ######################################################################################################################################################
///  类 unit  结构体     <类似 unit 的结构体>
/// ###################################################################################################################################################### 
/// 
/// 它类似于 (), 即 uint 元组
/// 
/// 
/// 【用途】: 
/// 
///         在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
///
/// 
/// struct AlwaysEqual;
/// 
/// fn main() {
///     let subject = AlwaysEqual;
/// }
/// 
/// 
/// ######################################################################################################################################################
/// 
///  字段 【引用】 其他 类型的 struct  
/// 
/// (即 struct 的某些字段 【引用】 了其他 外部类型数据) 
/// 
/// [需要加上 生命周期,  否则报错]: 因为 【引用】 了 外部数据, 而人家可能会被回收掉 从而造成 垂悬 【引用】 
/// 
/// 加上 生命周期 就可以规避这个问题
/// 
/// 
/// ###################################################################################################################################################### 
/// 
/// ------------------------------------
/// 错误示例
/// ------------------------------------
/// struct User {
///     active: bool, 
///     username: &str,    // --------------- missing lifetime specifier       引用，需要加上 life type
///     email: &str,       // --------------- missing lifetime specifier       引用，需要加上 life type
///     sign_in_count: u64,   
/// }
/// 
/// fn main() {
///     let user1 = User {
///         active: true, 
///         username: "someusername123", 
///         email: "someone@example.com", 
///         sign_in_count: 1, 
///     };
/// }
/// 
/// 
/// 
/// 
/// ######################################################################################################################################################
///  打印结构体
/// ###################################################################################################################################################### 
/// 
/// 
/// #[derive(Debug)]
/// struct Rectangle {
///     width: u32, 
///     height: u32, 
/// }
/// 
/// fn main() {
///     let rect1 = Rectangle {
///         width: 30, 
///         height: 50, 
///     };
/// 
/// 
///     // println! 宏使用的是 【一个表达式的引用】,    dbg! 宏使用的是 【一个表达式的所有权】
///     //
///     // 【注意:】 调用 dbg! 宏会打印到标准错误控制台流(stderr), 与 println! 不同, 后者会打印到标准输出控制台流(stdout)
///     //
///     println!("rect1 is {:?}",  rect1);
/// 
///     println!("rect1 is {#:?}",  rect1);
/// }
/// 
/// 
/// ######################################################################################################################################################
///  打印结构体 使用 dbg! 宏调试
/// ###################################################################################################################################################### 
/// 
/// 
/// #[derive(Debug)]
/// struct Rectangle {
///     width: u32, 
///     height: u32, 
/// }
/// 
/// fn main() {
///     let scale = 2;
///     let rect1 = Rectangle {
///         width: dbg!(30 * scale),  // 放在表达式 30 * scale 周围, 因为 dbg! 返回表达式的值的所有权, 所以 width 字段将获得相同的值, 就像我们在那里没有 dbg!
///         height: 50, 
///     };
/// 
///     dbg!(&rect1);
/// }
/// 
/// 
/// 
/// ######################################################################################################################################################
///  结构体的方法 
/// ###################################################################################################################################################### 
/// 
/// 
/// ----------------------------------------
/// 
/// 1. 为结构体实现trait 为:
///  
///                 impl trait名称 for 结构体名称 {}
/// 
/// 
/// 2. 而 结构体的 方法为:
/// 
///                 impl 结构体名称 {}
/// 
/// ----------------------------------------
/// 
/// 
///  
/// 
/// #[derive(Debug)]
/// struct Rectangle {
///     width: u32, 
///     height: u32, 
/// }
/// 
/// 
/// // 所有在 impl 块中定义的函数被称为 关联函数
/// //
/// // 因为它们与 impl 后面命名的类型相关
/// //
/// impl Rectangle {
/// 
///     // 方法的第一个参数必须有一个名为 self 《变量名》 的 【Self 类型】 的参数, 所以 Rust 让你在第一个参数位置上只用 self 这个名字来缩写
///     //
///     // 注意, 我们仍然需要在 self 前面使用 & 来表示这个方法借用了 Self 实例. 像我们在 rectangle: &Rectangle 中做的那样.
///     //
///     // 1、 方法可以选择获得 self 的所有权     【self】              ----------      (发生在: 用在当方法将 self 转换成别的实例的时候, 这时我们想要 防止 调用者 在转换之后 使用 原始的实例)
///     // 2、 或者像我们这里一样不可变地借用 self  【&self】           ----------      (发生在: 我们并不想获取所有权, 只希望能够读取结构体中的数据, 而不是写入)
///     // 3、 或者可变地借用 self, 就跟其他参数一样  【&mut self】     ----------      (发生在: 想要在方法中改变调用方法的实例)
///     //
///     // 
///     //
///     //          【&self 其实是 self: &Self 的缩写】
///     //
///     //
///     // 在一个 impl 块中, Self 类型是 impl 块的类型的别名,  这里就是  Rectangle 的别名
///     //
///     //  这里选择 &self 的理由跟在函数版本中使用 &Rectangle 是相同的:      我们并不想获取所有权, 只希望能够读取结构体中的数据, 而不是写入.
///     //
///     fn area(&self) -> u32 {
///         self.width * self.height
///     }
/// }
/// 
/// fn main() {
///     let rect1 = Rectangle {
///         width: 30, 
///         height: 50, 
///     };
/// 
///     println!(
///         "The area of the rectangle is {} square pixels.", 
///         rect1.area()
///     );
/// }
/// 
/// 
/// ######################################################################################################################################################
///  每个结构体都允许拥有多个 impl 块
/// ###################################################################################################################################################### 
/// 
/// 
/// impl Rectangle {
///     fn area(&self) -> u32 {
///         self.width * self.height
///     }
/// }
/// 
/// impl Rectangle {
///     fn can_hold(&self,  other: &Rectangle) -> bool {
///         self.width > other.width && self.height > other.height
///     }
/// }
