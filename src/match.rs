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







