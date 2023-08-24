///
/// 
/// ######################################################################################################################################################
///  泛型 的定义和使用  
/// ###################################################################################################################################################### 
/// 
/// 
/// // T 类型字段的 Point
/// struct Point<T> {
///     x: T, 
///     y: T, 
/// }
/// 
/// // 实现针对 T 类型的的 Point 的方法      (【注意:】我们可以为泛型参数选择一个与结构体定义中声明的泛型参数所不同的名称, 不过依照惯例使用了相同的名称)
/// //
/// impl<T> Point<T> { // 分别 在 ipml 和 结构体类型名 后面写上 (lifetime 注解也是一样...)
/// 
///     fn x(&self) -> &T {
///         &self.x
///     }
/// }
/// 
/// fn main() {
///     let p = Point { x: 5,  y: 10 };
/// 
///     println!("p.x = {}",  p.x());
/// }
/// 
/// 
/// 
/// ######################################################################################################################################################
///  结构体定义中的 泛型类型参数 并不总是 与 结构体方法签名 中使用的泛型是 同一类型 
/// ###################################################################################################################################################### 
/// 
/// 
/// 
/// 
/// 
/// struct Point<X1,  Y1> {
///     x: X1, 
///     y: Y1, 
/// }
/// 
/// // 这个方法用 self 的 Point 类型的 x 值(类型 X1)和参数的 Point 类型的 y 值(类型 Y2)来创建一个新 Point 类型的实例.
/// 
/// // 这个例子的目的是展示一些泛型通过 impl 声明而另一些通过方法定义声明的情况.
/// // 这里泛型参数 X1 和 Y1 声明于 impl 之后, 因为他们与结构体定义相对应.而泛型参数 X2 和 Y2 声明于 fn mixup 之后, 因为他们只是相对于方法本身的
/// //
/// impl<X1,  Y1> Point<X1,  Y1> {
///     fn mixup<X2,  Y2>(self,  other: Point<X2,  Y2>) -> Point<X1,  Y2> {
///         Point {
///             x: self.x, 
///             y: other.y, 
///         }
///     }
/// }
/// 
/// fn main() {
///     let p1 = Point { x: 5,  y: 10.4 };
///     let p2 = Point { x: "Hello",  y: 'c' };
/// 
///     let p3 = p1.mixup(p2);
/// 
///     println!("p3.x = {},  p3.y = {}",  p3.x,  p3.y);
/// }
///
/// 
/// 
/// ######################################################################################################################################################
///  lifetime
/// 
/// (每一个引用都有一个 lifetime,  为某些引用加上 lifetime注解 是提示 编译器 哪些 引用的 lifetime 是一致的,  如: x: &'a str 和 y: &'a str 是一致的,  不要把 y 提前清理掉了. )
/// ######################################################################################################################################################  
/// 
/// 
/// 【生命周期参数注解位于引用的 & 之后】
/// 
///  &i32        // 引用
///  &'a i32     // 带有显式生命周期的引用
///  &'a mut i32 // 带有显式生命周期的可变引用
/// 
/// 
/// 
/// -----------------------------------------------------------
/// 函数签名中的生命周期注解
/// -----------------------------------------------------------
/// 
/// 
/// fn main() {
///     let string1 = String::from("abcd");
///     let string2 = "xyz";
/// 
///     let result = longest(string1.as_str(),  string2);
///     println!("The longest string is {}",  result);
/// }
/// 
/// 
/// // 两个参数和返回的引用存活的一样久
/// //
/// fn longest<'a>(x: &'a str,  y: &'a str) -> &'a str {
///     if x.len() > y.len() {
///         x
///     } else {
///         y
///     }
/// }
/// 
/// 
/// 
/// -----------------------------------------------------------
/// 结构体定义中的生命周期注解   
/// 
/// (当 字段为 引用时, 应该设置 lifetime ,  不然 可能出现 垂悬引用)
/// -----------------------------------------------------------
/// 
/// 
/// struct ImportantExcerpt<'a> {
///     part: &'a str,     // 声明了 part 字段的生命周期 应该和 赋值给它的 &str 一致.
/// }
/// 
/// fn main() {
///     let novel = String::from("Call me Ishmael. Some years ago...");
///     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
///     let i = ImportantExcerpt {
///         part: first_sentence,    // part 的 lifetime 和 first_sentence 一致.
///     };
/// }
/// 
/// 
/// 
/// ######################################################################################################################################################
///  静态生命周期    'static
/// 
/// 
/// 
/// ('static, 其生命周期能够存活于整个程序期间;    所有的字符串字面值都是 'static 的)
/// ######################################################################################################################################################  
/// 
/// 
/// #![allow(unused)]
/// fn main() {
///     let s: &'static str = "I have a static lifetime.";
/// }
///     
/// 
/// 
/// ######################################################################################################################################################
///  泛型类型参数、trait bounds 和生命周期 
/// ######################################################################################################################################################
/// 
/// 
/// 
/// 
///  fn main() {
///      let string1 = String::from("abcd");
///      let string2 = "xyz";
///  
///      let result = longest_with_an_announcement(
///          string1.as_str(), 
///          string2, 
///          "Today is someone's birthday!", 
///      );
///      println!("The longest string is {}",  result);
///  }
///  
///  use std::fmt::Display;
///  
///  fn longest_with_an_announcement<'a,  T>(
///      x: &'a str, 
///      y: &'a str, 
///      ann: T, 
///  ) -> &'a str
///  where
///      T: Display, 
///  {
///      println!("Announcement! {}",  ann);
///      if x.len() > y.len() {
///          x
///      } else {
///          y
///      }
///  }




