///
/// 
/// Rust 编译器需要知道每个函数的   返回类型    需要多少空间。   这意味着所有函数都必须返回一个具体类型。
/// 
/// 
/// 
/// 如果你有个像 Animal 那样的的 trait，则不能编写返回 Animal 的函数，因为其不同的实现将需要不同的内存量。
/// 
/// 
/// 
/// 解决方法：
/// 
///         相比于直接返回一个 trait 对象，我们的函数返回一个包含一些 Animal 的 Box
/// 
/// 
///         如果你的函数以这种方式返回指向堆的 trait 指针，则需要使用 dyn 关键字编写返回类型，例如 Box<dyn Animal>
/// 
/// 
/// 
/// 例如：
/// 
/// 
///
///         struct Sheep {}
///         struct Cow {}
///     
///         trait Animal {
///             
///             fn noise(&self) -> &'static str;
///         }
///     
///         // 实现 `Sheep` 的 `Animal` trait
///         //
///         impl Animal for Sheep {
///             fn noise(&self) -> &'static str {
///                 "baaaaah!"
///             }
///         }
///     
///         // 实现 `Cow` 的 `Animal` trait
///         //
///         impl Animal for Cow {
///             fn noise(&self) -> &'static str {
///                 "moooooo!"
///             }
///         }
///     
///         // 返回一些实现 Animal 的结构体，但是在编译时我们不知道哪个结构体
///         //
///         fn random_animal(random_number: f64) -> Box<dyn Animal> {
///             if random_number < 0.5 {
///                 Box::new(Sheep {})
///             } else {
///                 Box::new(Cow {})
///             }
///         }
///     
///         fn main() {
///             let random_number = 0.234;
///             let animal = random_animal(random_number);
///             println!("You've randomly chosen an animal, and it says {}", animal.noise());
///         }
/// 
/// 
/// 
/// 






/// fn returns_closure() -> Box<dyn Fn(i32) -> i32> {  //其中 Fn(i32) -> i32 表示  闭包 Fn trait 的一个类型)    -----------------  注意 【关键字  Fn】 是指 闭包 trait
///     Box::new(|x| x + 1)
/// }
/// 
/// 
/// let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));    
/// 
/// 

