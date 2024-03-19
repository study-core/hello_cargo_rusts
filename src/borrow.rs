/// 【引用】 和 【借用】
/// 
/// 
/// 
/// 获取变量的 引用，称之为 借用
/// 
/// 
/// & 符号即是引用，它们允许你 【使用值】，但是 【不获取所有权】：
/// 
/// 
/// 
///     let s1 = String::from("hello");
/// 
///     let len = calculate_length(&s1);          -----------------       &s1 语法，我们创建了一个指向 s1 的引用，但是并不拥有它
/// 
///     println!("The length of '{}' is {}.", s1, len);
/// 
/// 
/// 
/// 原则：  (类似  读写 mutex)
/// 
/// 1. 可变引用同时只能存在一个
/// 
/// 2. 可变引用与不可变引用 不能 同时存在