/// ######################################################################################################################################################
/// 
///  String
/// 
/// ######################################################################################################################################################
/// 
/// let s1 = "I love !";  这个 s1 的值是字符串字面量, 是【不可变】的
/// 
/// 当想要【可变字符】或者 当并非所有字符串的值都能在编写代码时就知道:例如, 要是想获取用户输入并存储该怎么办呢? 时用   【String 类】
/// 
/// (String 类,  这个类型管理被分配到堆上的数据, 所以能够存储在编译时未知大小的文本)
/// 
/// 如: 
fn string_from() -> &str {
    let s = String::from("hello");
    s.push_str(",  world!"); // push_str() 在字符串后追加字面值
    println!("{}",  s); // 将打印 `hello,  world!`
}

/// ######################################################################################################################################################
/// 
///  所有权转移例子
/// 
/// ######################################################################################################################################################
/// 
fn string_own_mv() -> () {
    let s1 = String::from("hello");  // 内存在 堆中; String 类型<不是  基础类型 哦>
    let s2 = s1;

    println!("{},  world!",  s1); // error[E0382]: borrow of moved value: `s1`. 即 s1 的所有权已经转移到 s2


    // 但是对于 基础类型 不适用 (基础类型是 值copy)

    let x = 5;   
    let y = x;  // 这时 x = 5 ,  y = 5 两个值 被压入 栈中 ...
    
}

/// ######################################################################################################################################################
/// 
///  深拷贝 (copy 堆数据)
/// 
/// String 类, 这种编译时 不知道大小的, 将被分配大 堆上
/// 
/// ######################################################################################################################################################
fn string_deep_copy() -> () {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {},  s2 = {}",  s1,  s2);
}

/// ######################################################################################################################################################
/// 
///  浅拷贝 (copy 栈数据)
/// 
/// 整型这样的在编译时已知大小的类型被整个存储在栈上, 所以拷贝其实际的值是快速的
/// 
/// ######################################################################################################################################################
fn value_shallow_copy() -> () {
    let x = 5;    // 基础类型
    let y = x;

    println!("x = {},  y = {}",  x,  y);
}

/// ######################################################################################################################################################
/// 
///  Copy  trait
/// 
/// ######################################################################################################################################################
/// 
/// Rust 有一个叫做 Copy trait 的特殊注解, 可以用在类似整型这样的存储在栈上的类型上.如果一个类型实现了 Copy trait, 那么一个旧的变量在将其赋值给其他变量后仍然可用
///
/// Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait.如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 注解
/// 
/// 1、 任何一组简单 标量值 的组合都可以实现 Copy
/// 
/// 2、 任何 不需要 分配内存 或 某种形式资源 的类型都可以实现 Copy
/// 
/// 一些有Copy的类型:
/// 
/// 1、所有整数类型, 比如 u32
////2、布尔类型, bool, 它的值是 true 和 false
////3、所有浮点数类型, 比如 f64
////4、字符类型, char
////5、元组, 当且仅当其包含的类型也【都】实现 Copy 的时候.    比如, (i32,  i32) 实现了 Copy,     但 (i32,  String) 就没有.
/// 


/// ######################################################################################################################################################
/// 
///  函数入参 及 所有权
/// 
/// ######################################################################################################################################################
fn own_mv_for_input_args() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里, 
                                    // 但 i32 是 Copy 的, 
                                    // 所以在后面可继续使用 x

} // 这里, x 先移出了作用域, 然后是 s.但因为 s 的值已被移走, 
  // 没有特殊之处
fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}",  some_string);
} // 这里, some_string 移出作用域并调用 `drop` 方法.
  // 占用的内存被释放
fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}",  some_integer);
} // 这里, some_integer 移出作用域.没有特殊之处

/// ######################################################################################################################################################
/// 
///  函数返参  及 所有权
/// 
/// ######################################################################################################################################################
fn own_mv_for_return_args() {
    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 转移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中, 
                                        // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃 [没有被 return 则将被丢弃].s2 也移出作用域, 但已被移走, 
  // 所以什么也不会发生.s1 离开作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 会将
                                             // 返回值移动给
                                             // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域.

    some_string                              // 返回 some_string 
                                             // 并【移出给调用的函数】    所有权被移出到函数外头了
                                             // 
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
                                                      // 

    a_string  // 返回 a_string 并移出给调用的函数
}


/// ######################################################################################################################################################
/// 
/// 引用、 借用
/// 
/// (创建一个引用的行为称为 借用)
/// 
/// ######################################################################################################################################################
/// 
/// 引用: 不用获取所有权就可以使用值
/// 
/// 
/// fn main() {
///     let s1 = String::from("hello");    ------> String 类型
/// 
///     let len = calculate_length(&s1);   -------> &String 类型
/// 
///     println!("The length of '{}' is {}.",  s1,  len);
/// }
/// 
/// fn calculate_length(s: &String) -> usize {
///     s.len()
/// }
/// 
/// 
/// ********************************************************************
/// 不可 更改
/// ********************************************************************
/// 
/// 正如变量默认是不可变的, 引用也一样.(默认)不允许修改引用的值.
/// 
/// fn main() {
///     let s = String::from("hello");
/// 
///     change(&s);
/// }
/// 
/// fn change(some_string: &String) {
///     some_string.push_str(",  world");  // 报错: cannot borrow `*some_string` as mutable,  as it is behind a `&` reference
/// }
/// 
/// 
/// 
/// 
/// ********************************************************************
/// 可 更改
/// 
/// 使用 可变引用  &mut 
/// 
/// (局限性: 只能有一个  &mut)
/// ********************************************************************
/// 
/// 首先, 我们必须将 s 改为 mut.然后在调用 change 函数的地方创建一个可变引用 &mut s, 并更新函数签名以接受一个可变引用 some_string: &mut String.
/// 
/// 这就非常清楚地表明, change 函数将改变【它所借用】的值.
/// 
/// fn main() {
///     let mut s = String::from("hello");
/// 
///     change(&mut s);
/// }
/// 
/// fn change(some_string: &mut String) {
///     some_string.push_str(",  world");
/// }
/// 
/// 
/// 
/// ********************************************************************
/// 只能有一个  可变引用   &mut   
/// 
/// 
/// (报错示例~)
/// ********************************************************************
/// 
/// 
///     let mut s = String::from("hello");
/// 
///     let r1 = &mut s;   // <读写>
///     let r2 = &mut s;   // r2 非法 <读写>
/// 
///     println!("{},  {}",  r1,  r2);  // 报错: cannot borrow `s` as mutable more than once at a time
/// 
/// ----------------------------------------------------------------
/// 
/// 
/// ********************************************************************
/// 同时 可变 与  不可变引用  也有问题 (读写问题)
/// ******************************************************************** 
/// 
///     let mut s = String::from("hello");
/// 
///     let r1 = &s; // 没问题  ---  不可变引用 <读>
///     let r2 = &s; // 没问题  ---  不可变引用 <读>
///     let r3 = &mut s; // 大问题 (可变引用) <读写>
/// 
///     println!("{},  {},  and {}",  r1,  r2,  r3);
/// 
/// 
/// ********************************************************************
/// 可以使用  {} 代码块 来规避  多 &mut 报错问题
/// ********************************************************************
/// 
///     let mut s = String::from("hello");
/// 
///     {
///         let r1 = &mut s; // <读写>
///     } // r1 在这里离开了作用域, 所以我们完全可以创建一个新的引用
/// 
///     let r2 = &mut s; // <读写>
/// 
/// 
/// ********************************************************************
/// 引用的作用域 从声明 的地方 开始一直 持续到 最后一次使用为止
/// ********************************************************************
/// 
/// 
///     let mut s = String::from("hello");
/// 
///     let r1 = &s; // 没问题
///     let r2 = &s; // 没问题
///     println!("{} and {}",  r1,  r2);
///     // 此位置之后 r1 和 r2 不再使用
/// 
///     let r3 = &mut s; // 没问题
///     println!("{}",  r3);
/// 
/// 
/// ********************************************************************
/// 
/// 悬垂引用
/// 
/// ********************************************************************
/// 
/// 使用 返回 非引用变量 (将非引用变量所有权转移出函数 来解决 悬垂引用)
/// 
/// 
/// fn main() {
///     let reference_to_nothing = dangle();  // 报错: missing lifetime specifier
/// }
/// 
/// fn dangle() -> &String { // dangle 返回一个字符串的引用
/// 
///     let s = String::from("hello"); // s 是一个新字符串
/// 
///     &s // 返回字符串 s 的引用
/// } // 这里 s 离开作用域并被丢弃.其内存被释放. 【但是 &s 被返回出去了】
///   
/// ------------------------------------------------------------
/// 
/// 使用 String 即可,  【s 的所有权被移动出去, 所以没有值被释放】
/// 
/// 
/// fn no_dangle() -> String {
///     let s = String::from("hello");
/// 
///     s  【s 的所有权被移动出去, 所以没有值被释放】
/// }
/// 
/// 
/// 
/// 
/// 
/// 传递 引用不代表 获得变量的所有权,  即: &String 只是 String 的引用, 但没有获取 String 的所有权,  而将 String 赋值给其他 String类型变量才是 所有权转移