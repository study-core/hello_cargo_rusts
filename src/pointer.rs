///
/// 
/// ######################################################################################################################################################
///  智能指针 的定义 及 使用 
/// ######################################################################################################################################################
/// 
/// 
/// 
/// Rust 标准库中定义了多种不同的智能指针，它们提供了多于引用的额外功能.
/// 
/// 
/// 【引用】和【智能指针】的一个额外的区别是：
///        
///         1. 引用是一类只借用数据的指针
///         2. 智能指针 拥有 他们指向的数据
/// 
/// 
/// 
/// 
/// 
/// 【智能指针  通常使用  结构体  实现】
/// 
/// 智能指针不同于结构体的地方在于其实现了 Deref 和 Drop trait
/// 
///            1. Deref trait 允许智能指针结构体实例表现的像引用一样，这样就可以编写既用于引用、又用于智能指针的代码
///             
///            2. Drop trait 允许我们自定义当智能指针离开作用域时运行的代码
/// 
/// 
/// 
/// 常见的 智能指针：
/// 
/// 
///         1、 Box<T>                  用于在堆上分配值
/// 
///         2、 Rc<T>                   一个引用计数类型，其数据可以有多个所有者 【用于 多个 读】
/// 
///         3、 Ref<T> 和 RefMut<T>     通过 RefCell<T> 访问   
///             (RefCell<T> 是一个在 【运行时】 而不是在编译时执行借用规则的类型)
/// 
/// 
/// 
/// 
/// ######################################################################################################################################################
///  Box<T>
/// 
///  (box 允许你将一个值放在堆上而不是栈上)
/// ######################################################################################################################################################
/// 
/// 
/// 
/// 除了数据被储存在堆上而不是栈上之外, box 没有性能损失, 不过也没有很多额外的功能。
/// 
/// 
/// 它们多用于如下场景：
///
///             1、当有一个在  编译时未知大小  的类型，而又想要在需要 确切大小的上下文中使用这个类型值的时候  (如： 递归类型 [无穷 递归下是无法得知类型大小的])
/// 
///             2、当有 大量数据 并希望在 确保数据不被拷贝 的情况下转移所有权的时候
/// 
///             3、当希望拥有一个值并只关心它的 类型是否实现了特定 trait  而不是 其具体类型的时候
/// 
/// 
/// 
///    fn main() {
///        let b = Box::new(5);
///        println!("b = {}", b);  // 数据是储存在栈上的那样访问 box 中的数据，正如任何拥有数据所有权的值那样，当像 b 这样的 box 在 main 的末尾离开作用域时，它将被释放
///                                //                                                                           (box 本身（位于栈上）和它所指向的数据（位于堆上）)
///    }
/// 
/// 
/// -----------------------------------------------
/// box 创建 递归类型
/// -----------------------------------------------
/// 
/// 
///  enum List {
///      Cons(i32, Box<List>),
///      Nil,
///  }
///  
///  use crate::List::{Cons, Nil};
///  
///  fn main() {
///      let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
///  }
/// 
/// 
/// 
/// -----------------------------------------------
/// 像引用一样使用 Box<T>
/// -----------------------------------------------
/// 
/// 
/// 先来说说引用 及 解引用 操作
/// 
/// fn main() {
///     let x = 5;
///     let y = &x;
/// 
///     assert_eq!(5, x);
///     assert_eq!(5, *y);  // 解引用
/// }
///
/// 再来说 box
/// 
/// 
/// fn main() {
///     let x = 5;
///     let y = Box::new(x);
/// 
///     assert_eq!(5, x);
///     assert_eq!(5, *y);  // 解引用
/// }
/// 
/// 
/// 
/// -----------------------------------------------
/// 自定义 Box<T>
/// -----------------------------------------------
/// 
/// 
///    use std::ops::Deref;
///    
///    impl<T> Deref for MyBox<T> {  // 实现  Deref trait
///    
///        type Target = T;                        // 定义了用于此 trait 的关联类型
///    
///        fn deref(&self) -> &Self::Target {      // 重载 (解引用) 关联方法
///            &self.0
///        }
///    }
///    
///    struct MyBox<T>(T);
///    
///    impl<T> MyBox<T> {
///        fn new(x: T) -> MyBox<T> {
///            MyBox(x)
///        }
///    }
///    
///    fn main() {
///        let x = 5;
///        let y = MyBox::new(x);
///    
///        assert_eq!(5, x);
///        assert_eq!(5, *y);
///    }
/// 
/// 
/// 
/// ######################################################################################################################################################
///  Rc<T> 
/// 
///  (引用计数智能指针,  【启用多所有权】) 【用于 多个 读】
/// ######################################################################################################################################################
/// 
///   【Rc<T> 只能用于     单线程场景】
/// 
/// 
///    enum List {
///        Cons(i32, Rc<List>),
///        Nil,
///    }
///    
///    use crate::List::{Cons, Nil};
///    use std::rc::Rc;
///    
///    fn main() {
///        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));      // 开启 引用计数指针  (可以被 其他 一直使用)
///        let b = Cons(3, Rc::clone(&a));                                 // Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝 (只会增加引用计数)
///        let c = Cons(4, Rc::clone(&a));                                 // (也可以调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用 Rc::clone)
///    
///        // (Rc<List> 的初始引用计数为 1，接着每次调用 clone，计数会增加 1。当 c 离开作用域时，计数减 1)
///    }
/// 
/// 
/// 
/// ######################################################################################################################################################
///  RefCell<T> 
/// 
///  (代表其数据的唯一的所有权,  【在运行时检查借用规则】) 【用于 多个 读】
/// ###################################################################################################################################################### 
/// 
/// 
///   【RefCell<T> 只能用于     单线程场景】
/// 
/// 
///                 对于 【引用】 和 【Box<T>】，借用规则的不可变性作用于 【编译时】。
///                 对于 【RefCell<T>】，这些不可变性作用于 【运行时】。
/// 
/// 
/// 【特定情况下，令一个值在其方法内部能够修改自身，而在其他代码中仍视为不可变，是很有用的】
/// 
/// 
/// 
/// 
/// 对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法，这属于 RefCell<T> 安全 API 的一部分。borrow 方法返回 Ref<T> 类型的智能指针，borrow_mut 方法返回 RefMut<T> 类型的智能指针。
/// 
/// 
/// 
/// RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。
/// 
/// 每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。
/// 当 Ref<T> 值离开作用域时，不可变借用计数减一。
/// 就像编译时借用规则一样，RefCell<T> 在任何时候只允许有 【多个不可变借用】 或 【一个可变借用】  (注意： 多个 可变借用， 就会报错：)
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// -------------------------------------------------------
/// Box  Rc  RefCell  的用法：
/// -------------------------------------------------------
/// 
///     
///     1、 Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
/// 
///     1、 Box<T> 允许在 【编译时】 执行 【不可变】 或 【可变】 借用检查；Rc<T>仅允许在 【编译时】 执行 【不可变】 借用检查；RefCell<T> 允许在 【运行时】 执行 【不可变】 或 【可变】 借用检查。
/// 
///     1、 因为 RefCell<T> 允许在运行时执行可变借用检查，所以 ( 我们可以在即便 RefCell<T> 【自身是不可变】 的情况下修改其内部的值 )。
/// 
/// 
/// 
/// -------------------------------------------------------
/// 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
/// -------------------------------------------------------
/// 
/// 【RefCell<T> 的一个常见用法是与 Rc<T> 结合】
/// 
/// 
/// 
///  #[derive(Debug)]
///  enum List {
///      Cons(Rc<RefCell<i32>>, Rc<List>),
///      Nil,
///  }
///  
///  use crate::List::{Cons, Nil};
///  use std::cell::RefCell;
///  use std::rc::Rc;
///  
///  fn main() {
///      let value = Rc::new(RefCell::new(5));                          // 这是一个 【不可变借用】 的引用计数指针 Rc
///  
///      let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));        
///  
///      let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
///      let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
///  
///      *value.borrow_mut() += 10;                                     // 直接用了 RefCell 的 borrow_mut()  {该方法返回 RefMut<T> 智能指针 } 去通过 【内不可变性】 改变 Rc 的值
///                                                                     // (注意： 直接改变 (用 引用 、解引用 那套) rc 的值， 编译报错的, 只能用 RefCell (内不可变性) 去改变 【不可变引用】 的值)    
///  
///      println!("a after = {:?}", a);
///      println!("b after = {:?}", b);
///      println!("c after = {:?}", c);
///  }
/// 
/// 
/// 
/// 
/// 
/// 
/// ######################################################################################################################################################
/// 内部可变性：不可变值的可变借用
/// ######################################################################################################################################################
/// 
/// 
/// 错误示例： 
/// 
/// 
///  fn main() {
///      let x = 5;
///      let y = &mut x;    // 不能将  【不可变】的 x 转换成 【可变引用】， 想做 则考虑 RefCell 
///  }
/// 
/// 
/// 
/// 
/// -------------------------------------------------------
/// 补充
/// -------------------------------------------------------
/// 
/// 【Mutex<T> 是一个线程安全版本的 RefCell<T>】






