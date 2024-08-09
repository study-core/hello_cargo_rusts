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
/// // 实现针对 T 类型的的 Point 的方法      (【注意:】在这里我们可以为泛型参数选择一个与结构体定义中声明的泛型参数所不同的名称, 不过依照惯例使用了相同的名称)
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
///  泛型类型参数、 trait bounds  和  生命周期 
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




/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// 
///                                     生命周期约束 (高等级特征界限 HRTB, Higher-Rank Trait Bounds)
/// 
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// 
/// 
/// 
/// 1、假设有两个引用 &'a i32 和 &'b i32，它们的生命周期分别是 ：
///             
///             【'a 和 'b，若 'a >= 'b，则可以定义 'a:'b】，表示 【'a 至少要活得跟 'b 一样久】
/// 
/// 
/// 例如：
/// 
///     代码定义一个结构体，它拥有两个引用字段，类型都是泛型 T，每个引用都拥有自己的生命周期，由于我们使用了生命周期约束 'b: 'a，因此 'b 必须活得比 'a 久，
///     也就是结构体中的 s 字段引用的值必须要比 r 字段引用的值活得要久。
/// 
/// struct DoubleRef<'a,'b:'a, T> {
///     r: &'a T,
///     s: &'b T
/// }
/// 
/// 
/// 
//// 2、当表示类型 T 必须比 'a 活得要久时
/// 
///  例如：
/// 
///     表示结构体字段 r 引用了 T，  因此 r 的生命周期 'a 必须要比 T 的生命周期更短(被引用者的生命周期必须要比引用长)
/// 
/// struct Ref<'a, T: 'a> {
///     r: &'a T
/// }
/// 
/// 
/// 
/// 
/// 
/// 又，例如：
/// 
///         例子中必须添加约束 'a: 'b 后，才能成功编译，因为 self.part 的生命周期与 self的生命周期一致，将 &'a 类型的生命周期强行转换为 &'b 类型，会报错，只有在 'a >= 'b 的情况下，'a 才能转换成 'b 。
/// 
/// 
/// 
/// 
/// struct ImportantExcerpt<'a> {
///     part: &'a str,
/// }
/// 
/// impl<'a: 'b, 'b> ImportantExcerpt<'a> {
///     fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
///         println!("Attention please: {}", announcement);
///         self.part
///     }
/// }
/// 
/// 
/// 




/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// 
///                                             &'static 和 T: 'static
/// 
///                                 (静态生命周期   和  比静态生命周期大的生命周期)
/// 
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// 
/// 
/// 
/// 1、  &'static 对于生命周期有着非常强的要求：  
///                 
///                     一个 引用 必须要活得跟剩下的程序一样久，才能被标注为 &'static
/// 
///                     &'static 生命周期针对的仅仅是引用，而不是持有该引用的变量，对于变量来说，还是要遵循相应的作用域规则
/// 
/// 
/// 
/// 
/// 
/// 
/// 2、  T: 'static 与 &'static 有相同的约束：
/// 
///                     T 必须活得和程序一样久
/// 
/// 





/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// 
///                                             三条消除 lifetime  规则
/// 
/// 
/// 
///                             编译器使用三条消除规则来确定哪些场景不需要显式地去标注生命周期
/// 
/// ######################################################################################################################################################
/// ######################################################################################################################################################
///
/// *****************************************
/// *****************************************
/// 
/// 1. 每一个 【引用】 参数都会获得独自的生命周期
/// 
/// *****************************************
/// *****************************************
///         
/// 
///         一个引用参数的函数就有一个生命周期标注: fn foo<'a>(x: &'a i32)，两个引用参数的有两个生命周期标注:fn foo<'a, 'b>(x: &'a i32, y: &'b i32), 依此类推
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// **********************************************************************************
/// **********************************************************************************
/// 
/// 2. 若 【只有】 一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期   -------------------      也就是所有返回值的生命周期都等于该输入生命周期
/// 
/// **********************************************************************************
/// **********************************************************************************
/// 
/// 
///        函数 fn foo(x: &i32) -> &i32；     x 参数 的生命周期会被自动赋给返回值 &i32，因此该函数等同于 fn foo<'a>(x: &'a i32) -> &'a i32
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// **********************************************************************************
/// **********************************************************************************
/// 
/// 3. 若存在多个输入生命周期，且其中一个是 &self 或 &mut self    -----------------       则 &self 的生命周期被赋给所有的输出生命周期
/// 
/// **********************************************************************************
/// **********************************************************************************
/// 
/// 
///         (例如第三条规则，若一个方法，它的返回值的生命周期就是跟参数 &self 的不一样怎么办？总不能强迫我返回的值总是和 &self 活得一样久吧？! 问得好，答案很简单：    ---------------    手动标注生命周期)
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 例如：
/// 
/// 
///         fn first_word(s: &str) -> &str       
/// 
/// 
///         编译器会自动编成:            fn first_word<'a>(s: &'a str) -> &str     -------------  【根据 第一条规则】
///         
///         编译器会自动编成:            fn first_word<'a>(s: &'a str) -> &'a str     -------------  【根据 第一条规则】
///         
/// 
/// 
/// 又如：
/// 
/// 
///         fn longest(x: &str, y: &str) -> &str
/// 
/// 
///         编译器会自动编成:            fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str     -------------  【根据 第一条规则】
/// 
///         (【第二条规则】却无法被使用，因为输入生命周期有两个，【第三条规则】也不符合，因为它是函数，不是方法，因此没有 &self 参数，最终   报错)
/// 
///         (解决，需要手动标注，生命周期， 如：  fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str   或者  fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'b str  )
/// 







