///
/// 
/// 
/// ######################################################################################################################################################
///  模式匹配的使用
/// ###################################################################################################################################################### 
/// 
/// 
/// 
/// 
/// enum Coin {
///     Penny,
///     Nickel,
///     Dime,
///     Quarter,
/// }
/// 
/// fn value_in_cents(coin: Coin) -> u8 {
///     match coin {
///         Coin::Penny => 1,
///         Coin::Nickel => 5,
///         Coin::Dime => 10,
///         Coin::Quarter => 25,
///     }
/// }
/// 
/// fn main() {}
/// 
/// 
/// ######################################################################################################################################################
///  绑定值的模式
/// ###################################################################################################################################################### 
/// 
/// 
/// 
/// #[derive(Debug)]
/// enum UsState {
///     Alabama,
///     Alaska,
///     // --snip--
/// }
/// 
/// enum Coin {
///     Penny,
///     Nickel,
///     Dime,
///     Quarter(UsState),
/// }
/// 
/// fn value_in_cents(coin: Coin) -> u8 {
///     match coin {
///         Coin::Penny => 1,
///         Coin::Nickel => 5,
///         Coin::Dime => 10,
///         Coin::Quarter(state) => {
///             println!("State quarter from {:?}!", state);
///             25
///         }
///     }
/// }
/// 
/// fn main() {
///     value_in_cents(Coin::Quarter(UsState::Alaska));
/// }
/// 
/// 
/// ######################################################################################################################################################
///  if let
/// ###################################################################################################################################################### 
/// 
///  if let 语法让我们以一种不那么冗长的方式结合 if 和 let，来处理只匹配一个模式的值而忽略其他模式的情况
/// 
///  (可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值)
/// ----------------------------------------------------------------------------------------------
/// 
/// 
/// 将：
/// 
/// fn main() {
///     let config_max = Some(3u8);
///     match config_max {
///         Some(max) => println!("The maximum is configured to be {}", max),
///         _ => (),
///     }
/// }
/// 
/// 改成:
/// 
/// 
/// fn main() {
///     let config_max = Some(3u8);
///     if let Some(max) = config_max {
///         println!("The maximum is configured to be {}", max);
///     }
/// }
/// 
/// 
/// ----------------------------------------------------------------------------------------------
/// 
/// 下列的两种写法是 等价的:
/// 
/// 
/// #[derive(Debug)]
/// enum UsState {
///     Alabama,
///     Alaska,
///     // --snip--
/// }
/// 
/// enum Coin {
///     Penny,
///     Nickel,
///     Dime,
///     Quarter(UsState),
/// }
/// 
/// fn main() {
///     let coin = Coin::Penny;
///     let mut count = 0;
///     match coin {
///         Coin::Quarter(state) => println!("State quarter from {:?}!", state),
///         _ => count += 1,
///     }
/// }
/// 
/// 
/// 和:
/// 
/// 
/// #[derive(Debug)]
/// enum UsState {
///     Alabama,
///     Alaska,
///     // --snip--
/// }
/// 
/// enum Coin {
///     Penny,
///     Nickel,
///     Dime,
///     Quarter(UsState),
/// }
/// 
/// fn main() {
///     let coin = Coin::Penny;
///     let mut count = 0;
///     if let Coin::Quarter(state) = coin {
///         println!("State quarter from {:?}!", state);
///     } else {
///         count += 1;
///     }
/// }
/// 
/// 
/// ----------------------------------------------------------------------------------------------
/// 
/// 
/// 还可以：
/// 
/// 
///
/// fn main() {
/// 
///     // 3 个 condition
///     //
///     let favorite_color: Option<&str> = None;
///     let is_tuesday = false;
///     let age: Result<u8, _> = "34".parse();
/// 
///     if let Some(color) = favorite_color {  // 先匹配 favorite_color
/// 
///         println!("Using your favorite color, {color}, as the background");
/// 
///     } else if is_tuesday {                  // 再判断 is_tuesday
///         println!("Tuesday is green day!");
/// 
///     } else if let Ok(age) = age {           // 再匹配 age
///         if age > 30 {
///             println!("Using purple as the background color");
///         } else {
///             println!("Using orange as the background color");
///         }
///     } else {
///         println!("Using blue as the background color");
///     }
/// }
/// 
/// 
/// 
/// ######################################################################################################################################################
///  while let
/// 
/// (只要模式匹配就一直进行 while 循环)
/// ###################################################################################################################################################### 
/// 
/// 
///  fn main() {
///      let mut stack = Vec::new();
///  
///      stack.push(1);
///      stack.push(2);
///      stack.push(3);
///  
///      while let Some(top) = stack.pop() { // 只要 pop 出来的内容 匹配 Some(x) ， 就一直循环
///          println!("{}", top);
///      }
///  }







