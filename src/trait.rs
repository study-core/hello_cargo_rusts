///
/// 
/// ######################################################################################################################################################
///  trait 的定义和使用  
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
///         format!("{},  by {} ({})",  self.headline,  self.author,  self.location)
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
///         format!("{}: {}",  self.username,  self.content)
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
///     // trait 的默认实现,  结构体要么保留, 要么重载该实现
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
///         format!("{}: {}",  self.username,  self.content)
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
///         format!("{},  by {} ({})",  self.headline,  self.author,  self.location)
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
///         format!("{}: {}",  self.username,  self.content)
///     }
/// }
/// 
/// pub fn notify(item: &impl Summary) {  // 表明  只要是 impl 了 trait Summary 的结构都可以作为当前函数入参.
///     println!("Breaking news! {}",  item.summarize());
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
///     println!("Breaking news! {}",  item.summarize());
/// }
/// 
/// 等价于
/// 
/// 【适用于短小的例子】
/// 
/// pub fn notify(item: &impl Summary) {  
///     println!("Breaking news! {}",  item.summarize());
/// }
/// 
/// 
/// ######################################################################################################################################################
///  通过  【 +   号】,  指定多个 trait bound 
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
/// fn some_function<T: Display + Clone,  U: Clone + Debug>(t: &T,  u: &U) -> i32 { unimplemented!() }
/// 
/// 简化成
/// 
/// fn some_function<T,  U>(t: &T,  u: &U) -> i32
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
/// 下面这种是 【错误】 示例:
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
///         format!("{},  by {} ({})",  self.headline,  self.author,  self.location)
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
///         format!("{}: {}",  self.username,  self.content)
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
///             location: String::from("Pittsburgh,  PA,  USA"), 
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
///                 "of course,  as you probably already know,  people", 
///             ), 
///             reply: false, 
///             retweet: false, 
///         }
///     }
/// }
/// 
/// ---------------------------------------------------------------------------
/// 
/// 下面是 【正确】 示例:
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
///         format!("{},  by {} ({})",  self.headline,  self.author,  self.location)
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
///         format!("{}: {}",  self.username,  self.content)
///     }
/// }
/// 
/// // 只返回了 trait Summary 的一种实现: Tweet
/// //
/// fn returns_summarizable() -> impl Summary {
///     Tweet {
///         username: String::from("horse_ebooks"), 
///         content: String::from(
///             "of course,  as you probably already know,  people", 
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
///     fn new(x: T,  y: T) -> Self {
/// 
///         Self { x,  y }   // Self 是一个 impl 块 类型的 [类型别名]
/// 
///     }
/// }
/// 
/// impl<T: Display + PartialOrd> Pair<T> {
///     fn cmp_display(&self) {
///         if self.x >= self.y {
///             println!("The largest member is x = {}",  self.x);
///         } else {
///             println!("The largest member is y = {}",  self.y);
///         }
///     }
/// }
/// 
/// 
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// ######################################################################################################################################################
///                                             
///                                             关联类型      和       默认类型参数
/// 
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// 
/// 
///  use std::ops::Add;
///  
///  struct Millimeters(u32);
///  struct Meters(u32);
///  
///  trait Add<Rhs=Self> {   // Rhs : right hand side ; Rhs=Self 表示: 如果实现 Add trait 时不指定 Rhs 的具体类型, Rhs 的类型将是默认的 Self 类型
/// 
///                          // Rhs=Self 叫做:       【默认类型参数】
///  
///      type Output;        // 关联类型: 与 trait 关联的  【类型占位符】,  在实现 trait 时, 必须指定具体的类型
///  
///      fn add(self,  rhs: Rhs) -> Self::Output;
///  }
///  
///  
///  impl Add<Meters> for Millimeters {
///      type Output = Millimeters;
///  
///      fn add(self,  other: Meters) -> Millimeters {
///          Millimeters(self.0 + (other.0 * 1000))
///      }
///  }
///  
/// 
/// 
/// ######################################################################################################################################################
///  同时  实现多个 具有同名方法  的 trait
/// ###################################################################################################################################################### 
/// 
/// 
/// 
/// trait Pilot {
///     fn fly(&self);
/// }
/// 
/// trait Wizard {
///     fn fly(&self);
/// }
/// 
/// struct Human;
/// 
/// impl Pilot for Human {                                  //  实现 trait Pilot 的 fn fly(&self)
///     fn fly(&self) {
///         println!("This is your captain speaking.");
///     }
/// }
/// 
/// impl Wizard for Human {                                 //  实现 trait Wizard 的 fn fly(&self)
///     fn fly(&self) {
///         println!("Up!");
///     }
/// }
/// 
/// impl Human {
///     fn fly(&self) {                                     //  实现 自己 的 fn fly(&self)
///         println!("*waving arms furiously*");
///     }
/// }
/// 
/// fn main() {
///     let person = Human;
///     person.fly();                                       // 【只能调用到  自己  的 fn fly(&self)】
/// }
/// 
/// 
/// ---------------------------------------------------------------------------
/// 
/// 如果想 都调用到 则用:  
/// 
/// 
/// 
/// 
///  fn main() {
///      let person = Human;
/// 
///      Pilot::fly(&person);
///      Wizard::fly(&person);
///      person.fly();
///  }
///  
///  ---------------------------------------------------------------------------
///  
/// 【 完全限定语法】
/// 
/// <Type as Trait>::function(receiver_if_method,  next_arg,  ...);
/// 
/// 
///  trait Animal {
///      fn baby_name() -> String;
///  }
///  
///  struct Dog;
///  
///  impl Dog {
///      fn baby_name() -> String {
///          String::from("Spot")
///      }
///  }
///  
///  impl Animal for Dog {
///      fn baby_name() -> String {
///          String::from("puppy")
///      }
///  }
///  
///  fn main() { 
///      println!("A baby dog is called a {}",  Dog::baby_name());                // Dog 的
///  
///      // println!("A baby dog is called a {}",  Animal::baby_name());          // 错的,  trait 的方法直接调,  编译器无法推断出 它的实现结构
///  
///      println!("A baby dog is called a {}",  <Dog as Animal>::baby_name());    // trait Animal 的
///  }
///  
///  
/// 
/// 
/// ######################################################################################################################################################
///  在 [外部类型] 上实现 [外部 trait]
/// 
/// 
/// 如: Vec 上 实现 trait Display  (Vec 和 Display 及 我们当前代码所在, 相互为  [外部])
/// 
/// 解决方案:
/// 
///  (使用 newtype 模式 去包装 Vec,  然后在 newtype 上实现 Display)
/// ###################################################################################################################################################### 
///  
///  
///  
///  use std::fmt;
///  
///  struct Wrapper(Vec<String>);    // 定义 newtype : Wrapper(Vec<String>)   [newtype:  使用原有类型实现的新类型,  这里原有类型是 Vec]
///  
///  impl fmt::Display for Wrapper { // 再用 Wrapper 去实现 trait Display
/// 
///      fn fmt(&self,  f: &mut fmt::Formatter) -> fmt::Result {
///          write!(f,  "[{}]",  self.0.join(",  "))
///      }
///  }
///  
///  fn main() {
///      let w = Wrapper(vec![String::from("hello"),  String::from("world")]);
///      println!("w = {}",  w);
///  }
///  





