///
/// 
///                                             外部语言函数接口   (Foreign Function Interface,  FFI)
/// 
/// 
/// 
/// ######################################################################################################################################################
///  调用外部函数   extern
/// ###################################################################################################################################################### 
/// 
/// 
/// 有时你的 Rust 代码可能需要与其他语言编写的代码交互.为此 Rust 有一个关键字, extern, 有助于创建和使用 外部函数接口(Foreign Function Interface, FFI)
/// 
/// 
/// 
/// 
/// 示例: 如何集成 C 标准库中的 abs 函数
/// 
/// 
/// (extern 块中声明的函数在 Rust 代码中总是不安全的.因为其他语言不会强制执行 Rust 的规则且 Rust 无法检查它们)
/// 
/// 
/// 
///  extern "C" {    // 使用 rust 语法, 声明 C 的 abs 函数签名
///      fn abs(input: i32) -> i32;
///  }
///  
///  fn main() {
///      unsafe {
///          println!("Absolute value of -3 according to C: {}",  abs(-3));
///      }
///  }
/// 
/// 
/// 
/// 
/// 
/// ######################################################################################################################################################
///  创建一个允许其他语言调用 Rust 函数的接口
/// ######################################################################################################################################################
/// 
/// 也可以使用 extern 来创建一个允许其他语言调用 Rust 函数的接口
/// 
/// 
/// 
/// 
/// 不同于创建整个 extern 块, 就: (在 fn 关键字之前增加 extern 关键字并为相关函数指定所用到的 ABI) 
/// 
/// 还需增加 #[no_mangle] 注解来告诉 Rust 编译器不要 mangle 此函数的名称.   (mangle 就是 "碾压" 的意思，不同编译器会将 fn name 编成一堆特殊的字符)
/// 
/// 
/// Mangling 发生于当编译器将我们指定的函数名修改为不同的名称时, 这会增加用于其他编译过程的额外信息, 不过会使其名称更难以阅读.
/// 每一个编程语言的编译器都会以稍微不同的方式 mangle 函数名, 所以为了使 Rust 函数能在其他语言中指定, 必须禁用 Rust 编译器的 name mangling.
/// 
/// 
/// #[no_mangle]
/// pub extern "C" fn call_from_c() {
///     println!("Just called a Rust function from C!");
/// }
///
/// 
/// 然后在 C 中这样调用 rust 函数：
///
///
/// extern void call_from_c();
///
/// int main(void) {
///     call_from_c();
///     return 0;
/// }
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 

/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// 
/// 
///                                                                    更详细的写法
/// 
/// 
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// 
/// 【 C 中调用 Rust 】
/// 
/// 
/// rust 侧：
/// 
///         // src/lib.rs
///         #[no_mangle]
///         pub extern fn double_input(input: i32) -> i32 {
///             input * 2
///         }
/// 
/// 
/// c 侧：
/// 
/// 
///         // src/main.c
///         #include <stdint.h>
///         #include <stdio.h>
///         
///         extern int32_t double_input(int32_t input);
///         
///         int main() {
///             int input = 4;
///             int output = double_input(input);
///             printf("%d * 2 = %d\n", input, output);
///             return 0;
///         }
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 【 Rust 中调用 C 】
/// 
/// 
/// c 侧：
/// 
///         // src/double.c
///         int double_input(int input) {
///             return input * 2;
///         }
///
/// 
/// 
/// 
/// rust 侧：
/// 
/// 
///         // build.rs
///         extern crate cc;
///         
///         fn main() {
///             cc::Build::new()
///                 .file("src/double.c")
///                 .compile("libdouble.a");
///         }
/// 
/// 
///         // src/main.rs
///         extern crate libc;
///         
///         extern {
///             fn double_input(input: libc::c_int) -> libc::c_int;
///         }
///         
///         fn main() {
///             let input = 4;
///             let output = unsafe { double_input(input) };
///             println!("{} * 2 = {}", input, output);
///         }
/// 
/// 
/// 
/// 
/// 【 C++ 中调用 Rust 】
/// 
/// 
/// rust 侧：
/// 
/// 
///         // src/lib.rs
///         #[no_mangle]
///         pub extern fn double_input(input: i32) -> i32 {
///             input * 2
///         }
///
/// 
/// 
/// 
/// 
/// 
/// c++ 侧：
/// 
///         // src/example.h
///         #ifndef _EXAMPLE_H
///         #define _EXAMPLE_H
///         
///         #ifdef __cplusplus
///         extern "C"{
///         #endif
///         
///         int double_input(int input);
///         
///         #ifdef __cplusplus
///         }
///         #endif
///         #endif
/// 
/// 
/// 
/// 
/// 
/// 
///         // src/main.cpp
///         #include <iostream>
///         #include "example.h"
///         
///         using namespace std;
///         
///         int main() {
///           int input = 10;
///           int output = double_input(input);
///           cout<<input<<" * 2 = "<<output<<"\n";
///         
///           return 0;
///         }
/// 
/// 
/// 
/// 
/// 【 Rust 中调用 C++ 】
/// 
/// 
/// c++ 侧：
/// 
/// 
///         // src/triple.cpp
///         extern "C"
///         int triple_input(int input) {
///             return input * 3;
///         }
/// 
/// 
/// 
/// rust 侧：
/// 
/// 
///         // build.rs
///         extern crate cc;
///         
///         fn main() {
///             cc::Build::new()
///                 .file("src/triple.cpp")
///                 .cpp(true)
///                 .compile("libtriple.a");
///         }
///         
/// 
/// 
///         // src/maim.rs
///         extern crate libc;
///         
///         extern {
///             fn triple_input(input: libc::c_int) -> libc::c_int;
///         }
///         
///         fn main() {
///             let input = 4;
///             let output = unsafe { triple_input(input) };
///             println!("{} * 3 = {}", input, output);
///         }
/// 
/// 


