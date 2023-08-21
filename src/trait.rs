///
/// 
/// ######################################################################################################################################################
///  trait 的定义和使用  
/// ######################################################################################################################################################  
/// 
/// 
/// ----------------------------------------
/// 
/// 1. 为结构体实现trait 为：
///  
///                 impl trait名称 for 结构体名称 {}
/// 
/// 
/// 2. 而 结构体的 方法为：
/// 
///                 impl 结构体名称 {}
/// 
/// ----------------------------------------
/// 
/// 
/// pub trait Summary {
///     fn summarize(&self) -> String;
/// }
/// 
/// pub struct NewsArticle {
///     pub headline: String,
///     pub location: String,
///     pub author: String,
///     pub content: String,
/// }
/// 
/// impl Summary for NewsArticle {
///     fn summarize(&self) -> String {
///         format!("{}, by {} ({})", self.headline, self.author, self.location)
///     }
/// }
/// 
/// pub struct Tweet {
///     pub username: String,
///     pub content: String,
///     pub reply: bool,
///     pub retweet: bool,
/// }
/// 
/// impl Summary for Tweet {
///     fn summarize(&self) -> String {
///         format!("{}: {}", self.username, self.content)
///     }
/// }
/// 
/// 
/// 
/// ######################################################################################################################################################
///  使用 trait 的 默认方法实现  
/// ######################################################################################################################################################
/// 
/// 
/// pub trait Summary {
/// 
///     // trait 的默认实现， 结构体要么保留，要么重载该实现
///     fn summarize(&self) -> String {
///         String::from("(Read more...)")
///     }
/// }
/// 
/// pub struct NewsArticle {
///     pub headline: String,
///     pub location: String,
///     pub author: String,
///     pub content: String,
/// }
/// 
/// impl Summary for NewsArticle {}   // 这个 结构体 使用了 trait 的默认实现
/// 
/// pub struct Tweet {
///     pub username: String,
///     pub content: String,
///     pub reply: bool,
///     pub retweet: bool,
/// }
/// 
/// impl Summary for Tweet {
///     fn summarize(&self) -> String {
///         format!("{}: {}", self.username, self.content)
///     }
/// }
/// 
/// 
/// 
/// ######################################################################################################################################################
///  trait 作为参数  
/// ###################################################################################################################################################### 
/// 
/// 
/// pub trait Summary {
///     fn summarize(&self) -> String;
/// }
/// 
/// pub struct NewsArticle {
///     pub headline: String,
///     pub location: String,
///     pub author: String,
///     pub content: String,
/// }
/// 
/// impl Summary for NewsArticle {
///     fn summarize(&self) -> String {
///         format!("{}, by {} ({})", self.headline, self.author, self.location)
///     }
/// }
/// 
/// pub struct Tweet {
///     pub username: String,
///     pub content: String,
///     pub reply: bool,
///     pub retweet: bool,
/// }
/// 
/// impl Summary for Tweet {
///     fn summarize(&self) -> String {
///         format!("{}: {}", self.username, self.content)
///     }
/// }
/// 
/// pub fn notify(item: &impl Summary) {  // 表明  只要是 impl 了 trait Summary 的结构都可以作为当前函数入参.
///     println!("Breaking news! {}", item.summarize());
/// }
/// 
/// 
/// -------------------------------------------------------------------------
/// Trait Bound 语法
/// -------------------------------------------------------------------------
/// 
/// 使用和上述等价的语法
/// 
/// 【用于更复杂的场景】
/// 
/// pub fn notify<T: Summary>(item: &T) {
///     println!("Breaking news! {}", item.summarize());
/// }
/// 
/// 等价于
/// 
/// 【适用于短小的例子】
/// 
/// pub fn notify(item: &impl Summary) {  
///     println!("Breaking news! {}", item.summarize());
/// }
/// 
/// 
/// ######################################################################################################################################################
///  通过  【 +   号】， 指定多个 trait bound 
/// ######################################################################################################################################################
/// 
/// 
/// 表示接收 同时实现了  trait Summary 和 trait Display 的结构体作为函数入参:
/// 
/// 
///
/// pub fn notify(item: &(impl Summary + Display)) {}
/// 
/// 或者
/// 
/// pub fn notify<T: Summary + Display>(item: &T) {}
/// 
/// 
/// 
/// ######################################################################################################################################################
///  通过 【where】 简化 trait bound 
/// ######################################################################################################################################################
/// 
/// 
/// 将
/// 
/// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { unimplemented!() }
/// 
/// 简化成
/// 
/// fn some_function<T, U>(t: &T, u: &U) -> i32
/// where
///     T: Display + Clone,
///     U: Clone + Debug,
/// {
///     unimplemented!()
/// }
/// 
/// 
/// ######################################################################################################################################################
///  返回值为  trait
/// 
///  (只适用于返回单一类型的情况)
/// ######################################################################################################################################################
/// 
/// 
/// 下面这种是 错误示例:
/// 
/// pub trait Summary {
///     fn summarize(&self) -> String;
/// }
/// 
/// pub struct NewsArticle {
///     pub headline: String,
///     pub location: String,
///     pub author: String,
///     pub content: String,
/// }
/// 
/// impl Summary for NewsArticle {
///     fn summarize(&self) -> String {
///         format!("{}, by {} ({})", self.headline, self.author, self.location)
///     }
/// }
/// 
/// pub struct Tweet {
///     pub username: String,
///     pub content: String,
///     pub reply: bool,
///     pub retweet: bool,
/// }
/// 
/// impl Summary for Tweet {
///     fn summarize(&self) -> String {
///         format!("{}: {}", self.username, self.content)
///     }
/// }
/// 
/// 
/// // 不能返回 两种 可能性的 trait 实现
/// //
/// fn returns_summarizable(switch: bool) -> impl Summary {
///     if switch {
///         NewsArticle {
///             headline: String::from(
///                 "Penguins win the Stanley Cup Championship!",
///             ),
///             location: String::from("Pittsburgh, PA, USA"),
///             author: String::from("Iceburgh"),
///             content: String::from(
///                 "The Pittsburgh Penguins once again are the best \
///                  hockey team in the NHL.",
///             ),
///         }
///     } else {
///         Tweet {
///             username: String::from("horse_ebooks"),
///             content: String::from(
///                 "of course, as you probably already know, people",
///             ),
///             reply: false,
///             retweet: false,
///         }
///     }
/// }
/// 
/// ---------------------------------------------------------------------------
/// 
/// 正确的
/// 
/// 
/// 
/// pub trait Summary {
///     fn summarize(&self) -> String;
/// }
/// 
/// pub struct NewsArticle {
///     pub headline: String,
///     pub location: String,
///     pub author: String,
///     pub content: String,
/// }
/// 
/// impl Summary for NewsArticle {
///     fn summarize(&self) -> String {
///         format!("{}, by {} ({})", self.headline, self.author, self.location)
///     }
/// }
/// 
/// pub struct Tweet {
///     pub username: String,
///     pub content: String,
///     pub reply: bool,
///     pub retweet: bool,
/// }
/// 
/// impl Summary for Tweet {
///     fn summarize(&self) -> String {
///         format!("{}: {}", self.username, self.content)
///     }
/// }
/// 
/// // 只返回了 trait Summary 的一种实现: Tweet
/// //
/// fn returns_summarizable() -> impl Summary {
///     Tweet {
///         username: String::from("horse_ebooks"),
///         content: String::from(
///             "of course, as you probably already know, people",
///         ),
///         reply: false,
///         retweet: false,
///     }
/// }
/// 
/// 
/// 
/// ######################################################################################################################################################
///  使用 trait bound 有条件地实现方法
/// ######################################################################################################################################################
/// 
/// 
/// 
/// use std::fmt::Display;
/// 
/// struct Pair<T> {
///     x: T,
///     y: T,
/// }
/// 
/// impl<T> Pair<T> {
///     fn new(x: T, y: T) -> Self {
/// 
///         Self { x, y }   // Self 是一个 impl 块 类型的 [类型别名]
/// 
///     }
/// }
/// 
/// impl<T: Display + PartialOrd> Pair<T> {
///     fn cmp_display(&self) {
///         if self.x >= self.y {
///             println!("The largest member is x = {}", self.x);
///         } else {
///             println!("The largest member is y = {}", self.y);
///         }
///     }
/// }









