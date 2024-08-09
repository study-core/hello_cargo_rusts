///
/// ######################################################################################################################################################
///  闭包 的定义和使用  
/// ######################################################################################################################################################  
/// 
/// 闭包语法类似于 函数
/// 
/// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
/// let add_one_v2 = |x: u32| -> u32 { x + 1 };
/// let add_one_v3 = |x|             { x + 1 };
/// let add_one_v4 = |x|               x + 1  ;
///
/// 
/// 
/// ######################################################################################################################################################
/// 闭包类型推断和注解
/// ######################################################################################################################################################
/// 
/// 
///  use std::thread;
///  use std::time::Duration;
///  
///  fn generate_workout(intensity: u32,  random_number: u32) {
///      
/// 
///      let expensive_closure = |num: u32| -> u32 {   // 将闭包赋值给 变量
///          println!("calculating slowly...");
///          thread::sleep(Duration::from_secs(2));
///          num
///      };
///  
///      if intensity < 25 {
///          println!("Today,  do {} pushups!",  expensive_closure(intensity));
///          println!("Next,  do {} situps!",  expensive_closure(intensity));
///      } else {
///          if random_number == 3 {
///              println!("Take a break today! Remember to stay hydrated!");
///          } else {
///              println!(
///                  "Today,  run for {} minutes!", 
///                  expensive_closure(intensity)
///              );
///          }
///      }
///  }
///  
///  fn main() {
///      let simulated_user_specified_value = 10;
///      let simulated_random_number = 7;
///  
///      generate_workout(simulated_user_specified_value,  simulated_random_number);
///  }
/// 
/// 
/// 
/// 
/// ######################################################################################################################################################
/// 闭包 捕获 引用 或者 移动 所有权
/// ######################################################################################################################################################
/// 
/// 
/// 
/// *********************************
///  捕获  【不可变引用的闭包】:      *        Fn ?
/// *********************************
/// 
/// 
/// fn main() {
///     let list = vec![1,  2,  3];
///     println!("Before defining closure: {:?}",  list);
/// 
///     let only_borrows = || println!("From closure: {:?}",  list);  // 捕获  【不可变引用的闭包】
/// 
///     println!("Before calling closure: {:?}",  list);
///     only_borrows();
///     println!("After calling closure: {:?}",  list);
/// }
/// 
/// 
/// *******************************
///  捕获  【可变引用的闭包】:      *          FnMut ?
/// *******************************
/// 
/// 
/// fn main() {
///     let mut list = vec![1,  2,  3];
///     println!("Before defining closure: {:?}",  list);
/// 
///     let mut borrows_mutably = || list.push(7);  // 捕获  【可变引用的闭包】
/// 
///     borrows_mutably();
///     println!("After calling closure: {:?}",  list);
/// }
/// 
/// 
/// *********************************
///  捕获  【获取所有权的闭包】:      *        FnOnce ?
/// ********************************* 
/// 
/// (使用  move 关键字 转移所有权)
/// 
/// use std::thread;
/// 
/// fn main() {
///     let list = vec![1,  2,  3];
///     println!("Before defining closure: {:?}",  list);
/// 
///     thread::spawn(move || println!("From thread: {:?}",  list))  // 捕获  【获取所有权的闭包】   使用  move 关键字 转移所有权
///         .join()
///         .unwrap();
/// }
/// 
/// 
/// 
/// 
/// 
/// ######################################################################################################################################################
///  将被捕获的值移出闭包  和  Fn trait  
/// ######################################################################################################################################################  
/// 
/// 
///   1、 FnOnce:   适用于能被【调用一次】的闭包,  所有闭包都至少实现了这个 trait,  因为所有闭包都能被调用. 
///                 (一个    [会]    将捕获的值移出闭包体的闭包只实现 FnOnce trait,  这是因为它只能 【被调用一次】.)
/// 
///   2、 FnMut:    适用于    [不会]    将捕获的值移出闭包体的闭包,  但它可能会修改被捕获的值. 这类闭包可以【被调用多次】.
/// 
///   3、 Fn:       适用于     [既不将]     被捕获的值移出闭包体     [也不修改]    被捕获的值的闭包,  【当然也包括 不从环境中捕获值的闭包】. 
///                 ((( 这类闭包可以被调用多次而不改变它们的环境,  这在会多次并发调用闭包的场景中十分重要 )))
/// 
/// 三者的关系：
/// 
/// Fn 的前提是实现 FnMut，FnMut 的前提是实现 FnOnce，因此要实现 Fn 就要同时实现 FnMut 和 FnOnce。
/// 
/// Fn 获取 &self，FnMut 获取 &mut self，而 FnOnce 获取 self。 
/// 
/// 在实际项目中，建议先使用 Fn 特征，然后编译器会告诉你正误以及该如何选择。
/// 
/// 
/// 注意： move 关键字对于 FnOnce 特征的重要性，但是实际上使用了 move 的闭包依然可能实现了 Fn 或 FnMut 特征。
/// (原因： 一个闭包实现了哪种 Fn 特征取决于该闭包 【如何使用】 被捕获的变量，而不是取决于闭包 【如何捕获】 它们。
/// 【跟是否使用 move 没有必然联系】。move 本身强调的就是后者，闭包如何捕获变量)
/// ----------------------------------------------------------------------------------------------------------------------
/// 
/// 
///  impl<T> Option<T> {
///      pub fn unwrap_or_else<F>(self,  f: F) -> T
///      where
///          F: FnOnce() -> T
///      {
///          match self {
///              Some(x) => x, 
///              None => f(), 
///          }
///      }
///  }
/// 
/// 



/// 
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// 
///                              move      关键字
/// 
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// 
/// 
/// 关键字move的作用是将所引用的变量的所有权转移至闭包内，通常用于使闭包的生命周期大于所捕获的变量的原生命周期（例如将闭包返回或移至其他线程）
/// 
/// 
/// 
