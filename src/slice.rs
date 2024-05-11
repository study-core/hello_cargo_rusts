/// ********************************************************************
/// String字面量  本质上就是  slice,   "abc" 就是 slice 就是 &str  [&str 是 【不可变引用】]
/// ********************************************************************
/// 
/// 
///     let s = String::from("hello world");  --------------->  String 类型；    "hello world" 是 &str 类型
/// 
///     let hello = &s[0..5];
///     let world = &s[6..11];

/// ********************************************************************
///  错误的示例
/// ********************************************************************
/// 
/// 
///   fn main() {
///       let mut s = String::from("hello world");  // 可变 引用 s   【错误】
///   
///       let word = first_word(&s);  // 这里  不可变 String 引用 所有权转移到 word 
///   
///       s.clear(); // 错误! 借用规则, 当拥有某值的不可变引用时, 就不能再获取一个可变引用
///   
///       println!("the first word is: {}",  word);
///   }
///   
///   fn first_word(s: &String) -> &str { // 返回 不可变的 String 引用: slice,  类型为  &str
///       let bytes = s.as_bytes();
///   
///       for (i,  &item) in bytes.iter().enumerate() {
///           if item == b' ' {
///               return &s[0..i];
///           }
///       }
///   
///       &s[..]
///   }
/// 
/// 
/// 
/// ********************************************************************
/// 
///  字符串字面值就是 slice
///
/// ********************************************************************
/// 
/// slice 的类型是 &str
/// 
/// 它是一个指向二进制程序 特定位置的 slice
/// 
/// &str 是一个不可变引用
/// 
/// 
/// 
/// let s = "Hello,  world!"; // "Hello,  world" 本质是一个 slice,  类型为 &str
/// 
/// -------------------------------------------------
/// 最终, 写成下面的:
/// 
/// 
/// fn main() {
///     let my_string = String::from("hello world");
/// 
///     // `first_word` 适用于 `String`(的 slice), 部分或全部
///     let word = first_word(&my_string[0..6]);
///     let word = first_word(&my_string[..]);
///     // `first_word` 也适用于 `String` 的引用, 
///     // 这等价于整个 `String` 的 slice
///     let word = first_word(&my_string);
/// 
///     let my_string_literal = "hello world";
/// 
///     // `first_word` 适用于字符串字面值, 部分或全部
///     let word = first_word(&my_string_literal[0..6]);
///     let word = first_word(&my_string_literal[..]);
/// 
///     // 因为字符串字面值已经 **是** 字符串 slice 了, 
///     // 这也是适用的, 无需 slice 语法!
///     let word = first_word(my_string_literal);
/// }
/// fn first_word(s: &str) -> &str {
///     let bytes = s.as_bytes();
/// 
///     for (i,  &item) in bytes.iter().enumerate() {
///         if item == b' ' {
///             return &s[0..i];
///         }
///     }
/// 
///     &s[..]
/// }
/// -------------------------------------------------
/// 
/// 数组
/// 
/// let a: [i32; 5] = [1,  2,  3,  4,  5];
/// 
/// 或者
/// 
/// let a = [1,  2,  3,  4,  5];
/// 
/// 切片  (类型: &[i32])
/// 
/// let slice = &a[1..3];
///
/// assert_eq!(slice,  &[2,  3]);
/// 
/// 