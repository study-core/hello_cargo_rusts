/// 【注意】:Rust 出于安全的考虑, 默认情况下, 所有的类型都是 【私有化】 的, 包括函数、方法、结构体、枚举、常量, 是的, 就连 【模块本身】 也是私有化的.
/// 
/// 
///  但是  enum 的字段可是 pub 的哦， struct 的字段也是??
/// 
/// 
/// ######################################################################################################################################################
///  包
/// ###################################################################################################################################################### 
/// 
/// 
/// 
/// 包(package)是提供一系列功能的一个或者多个 crate.一个包会包含一个 Cargo.toml 文件.
/// 
/// 
/// 包中可以包含至多一个库 crate 【library crate】
/// 
/// 包中可以包含任意多个二进制 crate 【binary crate】 [入口函数 main()], 但是必须至少包含一个 crate(无论是 library crate 的还是 binary crate)
/// 
/// 
/// 
/// 
/// 
/// ######################################################################################################################################################
///  crate
/// ###################################################################################################################################################### 
/// 
/// 
/// 
///                 Rust编译器只接受一个.rs文件作为输入，并且只生成一个crate。这一点要牢记。
///
///                 生成的crate分两种，源文件中有main函数会生成可执行文件，无main函数则生成库。
/// 
/// 
/// 
/// crate 有两种形式: binary crate 和 library crate
/// 
/// 【binary crate】 二进制项:
/// 
///                  (有入口函数 main()) 可以被编译为可执行程序, 比如:一个命令行程序或者一个服务器.它们必须有一个 main 函数来定义当程序被执行的时候所需要做的事情.
///                     (目前我们所创建的 crate 都是二进制项.)
///
///【library crate】 库: 
/// 
///                 并没有 main 函数, 它们也不会编译为可执行程序, 它们提供一些诸如函数之类的东西, 使其他项目也能使用这些东西.
///                     (大多数时间 开发者说的 crate 指的都是 库)
/// 
/// 
/// ---------------------------------------------------------------------------------------------------
/// 
/// 
/// 
/// cargo 遵循的一个约定:
/// 
/// src/main.rs 是一个与包同名的 【library crate】 的 crate 根.
/// 
/// src/lib.rs, 是一个与包同名的 【binary crate】 的 crate 根.
/// 
/// (如果一个包同时含有 src/main.rs 和 src/lib.rs, 则它有两个 crate :  一个 【binary crate】 和 一个 【library crate】, 且名字都与包相同.)
/// 
/// 
/// crate root 是一个源文件, Rust 编译器以它为起始点, 并构成你的 crate 的根模块.
/// 
/// 
/// 
/// 
/// 
/// ---------------------------------------------------------------------------------------------------
/// 
/// 通过将文件放在 src/bin 目录下, 一个包可以拥有多个二进制 crate :  每个 src/bin 下的文件都会被编译成一个独立的二进制 crate.
/// 
/// 
/// ######################################################################################################################################################
///  mod  【默认是私有的】         
/// 
/// mod front_of_house; 告诉 Rust 从另一个模块 front_of_house 同名的文件 (front_of_house.rs) 中加载该模块的内容到当前模块中
/// ###################################################################################################################################################### 
/// 
/// 
/// 【注意】
///     
///     ++++++++++++++++++++++++++++++++++++++++++++++++++++++++
///     ++++++++++++++++++++++++++++++++++++++++++++++++++++++++
///     注意你只需在模块树中的某处  【使用一次 mod 声明】  就可以   加载这个文件
///     ++++++++++++++++++++++++++++++++++++++++++++++++++++++++
///     ++++++++++++++++++++++++++++++++++++++++++++++++++++++++
/// 
/// 
/// 
///     一旦编译器知道了这个文件是项目的一部分(并且通过 mod 语句的位置知道了代码在模块树中的位置), 
///     项目中的其他文件应该使用其所声明的位置的路径来引用那个文件的代码, 
///     换句话说, mod 不是 你可能会在其他编程语言中看到的 "include" 操作
/// 
///         
///         ++++++++++++++++++++++++++++++++++++++++++++++++++++++++
///         ++++++++++++++++++++++++++++++++++++++++++++++++++++++++
///         ++++++++++++++++++++++++++++++++++++++++++++++++++++++++
/// 
///         rust 使用 mod 关键词用来【定义模块】和【引入模块】.
/// 
///         rust 使用 use 关键词用来【调整模块内容调用路径】.
/// 
///         ++++++++++++++++++++++++++++++++++++++++++++++++++++++++
///         ++++++++++++++++++++++++++++++++++++++++++++++++++++++++
///         ++++++++++++++++++++++++++++++++++++++++++++++++++++++++
/// 
/// 
/// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
/// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
/// 
/// 【注意:】               use 仅仅是在存在模块的前提下, 【调整】调用路径, 而没有引入模块的功能, 引入模块使用 mod
/// 
/// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
/// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

/// 
/// 【声明模块:】 
/// 
///     在 crate 根文件中, 你可以声明一个新模块；比如:【你用mod garden声明了一个叫做 garden 的模块. 编译器会在下列路径中寻找模块代码】:
/// 
///              1、内联, 在大括号中, 当 mod garden 后方不是一个分号而是一个大括号
///              2、在文件 src/garden.rs
///              3、在文件 src/garden/mod.rs
/// 
/// ---------------------------------------------------------------------------------------------------
/// 
/// 【声明子模块:】 
///     在除了 crate 根节点以外的其他文件中, 你可以定义子模块.比如, 你可能在src/garden.rs中定义了mod vegetables;.编译器会在以父模块命名的目录中寻找子模块代码:
/// 
///                1、内联, 在大括号中, 当 mod vegetables 后方不是一个分号而是一个大括号
///                2、在文件 src/garden/vegetables.rs
///                3、在文件 src/garden/vegetables/mod.rs
/// 
/// ---------------------------------------------------------------------------------------------------
/// 
/// 在当前  crate 中, 如 上述 vegetables 子模块有一个 Asparagus 类型,  我们可以通过: crate::garden::vegetables::Asparagus 找到.
/// 
/// 
/// ---------------------------------------------------------------------------------------------------
/// 
/// 
/// 【私有 vs 公用:】
///         一个模块里的代码默认对其父模块【私有】.
///         为了使一个模块公用, 应当在声明时使用【 pub mod】 替代 mod. 【如:  pub mod garden;  这样做是行告诉编译器 应该【包含】在 src/garden.rs 文件中发现的代码】
///         为了使一个公用模块内部的成员公用, 应当在声明前使用【pub】.
///
/// 
///  use crate::garden::vegetables::Asparagus;
///  
///  pub mod garden; 
///  
///  fn main() {
///      let plant = Asparagus {};
///      println!("I'm growing {:?}!",  plant);
///  }
/// 
/// ---------------------------------------------------------------------------------------------------
/// 
/// 【use 关键字: 】
///         在一个作用域内,  【use】 关键字创建了一个成员的快捷方式,  用来减少长路径的重复.
///         在任何可以引用 crate::garden::vegetables::Asparagus 的作用域,  你可以通过 use crate::garden::vegetables::Asparagus;
///         创建一个快捷方式,  然后你就可以在作用域中只写 Asparagus 来使用该类型.
/// 
/// 
/// ######################################################################################################################################################
///  use 只能创建 use 所在的特定作用域内的短路径
/// ###################################################################################################################################################### 
/// 
///  
/// 
/// 错误示例:
/// 
/// 
///         将 eat_at_restaurant 函数移动到了一个叫 customer 的子模块, 这又是一个不同于 use 语句的作用域, 所以函数体不能编译.
/// 
/// 
/// 
/// --- 定义在 main.rs 中
/// 
/// mod front_of_house {   ----------- crate 的 【子模块】   
/// 
///     pub mod hosting {   ----------- crate 的 【孙模块】
/// 
///         pub fn add_to_waitlist() {}
///     }
/// }
/// 
/// use crate::front_of_house::hosting;      -------- 在 main.rs 中 创建 孙模块的 【快捷方式】
/// 
/// mod customer {
/// 
///     pub fn eat_at_restaurant() {
/// 
///         // 在 crate 的【子模块】 customer 中， 不能使用 crate 【子模块】 front_of_house 的 【子模块】 hosting 的东西，【只能将 hosting 重新导出,  pub use】
/// 
///         hosting::add_to_waitlist();   // 报错: failed to resolve: use of undeclared crate or module `hosting`
///     }
/// }
/// 
/// 
/// ######################################################################################################################################################
///  pub use 重导出名称
/// ###################################################################################################################################################### 
/// 
/// 
/// 当外部的模块项 A 被引入到当前模块中时, 它的可见性自动被设置为私有的, 如果你希望允许其它外部代码引用我们的模块项 A, 那么可以对它进行再导出:
/// 
/// 
/// mod front_of_house {
///     pub mod hosting {
///         pub fn add_to_waitlist() {}
///     }
/// }
/// 
/// pub use crate::front_of_house::hosting;  // 引入 A , 并再次导出 A, 不然其他外部代码无法引入 A
/// 
/// pub fn eat_at_restaurant() {
///     hosting::add_to_waitlist();
///     hosting::add_to_waitlist();
///     hosting::add_to_waitlist();
/// }
/// 
/// 例如:  
/// 
/// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
/// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
/// 
/// 统一使用一个模块来提供对外的 API, 那该模块就可以引入其它模块中的 API, 然后进行再导出
/// 
///  最终对于用户来说, 所有的 API 都是由一个模块统一提供的
/// 
/// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
/// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
/// 
/// 
/// 
/// 
/// 
/// ######################################################################################################################################################
///  限制可见性                 pub 关键字的用法
/// ###################################################################################################################################################### 
/// 
/// 
/// pub(crate) 或 pub(in crate::a) 就是限制可见性语法
/// 
/// 【pub(crate)】 ----------------- 是限制在整个包内可见
/// 
/// 【 pub(in crate::a)】 ----------------- 是通过绝对路径, 限制在包内的某个模块内可见:
/// 
/// 
/// 1、pub 意味着可见性 无任何限制
/// 2、pub(crate) 表示在  当前包可见
/// 3、pub(self) 在  当前模块可见
/// 4、pub(super) 在 父模块可见
/// 5、pub(in <path>) 表示在 某个路径代表的模块中可见 (其中 path 必须是 父模块 或者 祖先模块)
/// 
/// 
/// 
/// 
/// 
/// 
/// // 一个名为 `my_mod` 的模块
/// mod my_mod {
/// 
/// 
///     // 模块中的项默认具有私有的可见性
///     fn private_function() {
///         println!("called `my_mod::private_function()`");
///     }
/// 
///     // 使用 `pub` 修饰语来改变默认可见性.
///     pub fn function() {
///         println!("called `my_mod::function()`");
///     }
/// 
///     // 在同一模块中, 项可以访问其它项, 即使它是私有的.
///     pub fn indirect_access() {
///         print!("called `my_mod::indirect_access()`,  that\n> ");
///         private_function();
///     }
/// 
///     // 模块也可以嵌套
///     pub mod nested {
/// 
///         pub fn function() {
///             println!("called `my_mod::nested::function()`");
///         }
/// 
///         #[allow(dead_code)]
///         fn private_function() {
///             println!("called `my_mod::nested::private_function()`");
///         }
/// 
///         // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见
///         // `path` 必须是   父模块(parent module   )或   祖先模块(ancestor module)
///         //
///         pub(in crate::my_mod) fn public_function_in_my_mod() {
///             print!("called `my_mod::nested::public_function_in_my_mod()`,  that\n > ");
///             public_function_in_nested()
///         }
/// 
///         // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见
///         //
///         pub(self) fn public_function_in_nested() {
///             println!("called `my_mod::nested::public_function_in_nested");
///         }
/// 
///         // 使用 `pub(super)` 语法定义的函数只在父模块中可见
///         //
///         pub(super) fn public_function_in_super_mod() {
///             println!("called my_mod::nested::public_function_in_super_mod");
///         }
///     }
/// 
///     pub fn call_public_function_in_my_mod() {
///         print!("called `my_mod::call_public_funcion_in_my_mod()`,  that\n> ");
///         nested::public_function_in_my_mod();
///         print!("> ");
///         nested::public_function_in_super_mod();
///     }
/// 
///     // `pub(crate)` 使得函数只在当前包中可见
///     pub(crate) fn public_function_in_crate() {
///         println!("called `my_mod::public_function_in_crate()");
///     }
/// 
///     // 嵌套模块的可见性遵循相同的规则
///     mod private_nested {
///         #[allow(dead_code)]
///         pub fn function() {
///             println!("called `my_mod::private_nested::function()`");
///         }
///     }
/// }
/// 
/// fn function() {
///     println!("called `function()`");
/// }

fn main() {

    // 模块机制消除了相同名字的项之间的歧义
    //
    function();
    my_mod::function();

    // 公有项, 包括 嵌套模块内的, 都可以在父模块外部访问
    //
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个 crate 中的任何地方访问
    //
    my_mod::public_function_in_crate();

    // pub(in path) 项只能在指定的模块中访问
    // 报错!函数 `public_function_in_my_mod` 是私有的
    //
    //my_mod::nested::public_function_in_my_mod();
    // 试一试 ^ 取消该行的注释

    // 模块的 私有项  不能直接访问, 即便它是嵌套在  公有模块  内部的

    // 报错!`private_function` 是私有的
    //
    //my_mod::private_function();
    // 试一试 ^ 取消此行注释

    // 报错!`private_function` 是私有的
    //
    //my_mod::nested::private_function();
    // 试一试 ^ 取消此行的注释

    // 报错! `private_nested` 是私有的
    //
    //my_mod::private_nested::function();
    // 试一试 ^ 取消此行的注释
}


/// ##########################################################################################################################
/// ##########################################################################################################################
/// ##########################################################################################################################
/// ##########################################################################################################################
/// ##########################################################################################################################
/// ##########################################################################################################################
/// 
/// 
///             rust 编译器只会接收 一个 rs 文件，并且只生成一个crate要么是 main.rs (bin crate) 要么是 lib.rs (lib crate)；
///                                               (生成的crate分两种，源文件中有main函数会生成可执行文件，无main函数则生成库。)
/// 
/// 
/// 
/// 
/// 
///             Rust的模块就是命名空间，用关键词mod表示。它的作用是把一个crate的代码划分成可管理的部分。
///             每一个crate都有一个顶层的匿名根命名空间, 根空间下面的命名空间可以任意嵌套，这样构成一个树形结构。
///             
/// 
///             Rust编译器给我提供了一个将单个文件拆成多个文件的机制。
/// 
///             编译器的机制决定，除了mod.rs外，每一个 [文件] 和 [目录 (配合 mod.rs ??)] 都是一个模块。
/// 
///             
/// 
///             【mod】 两个作用： 
///                 1.声明 mod ；
///                 2. 将其他子 mod 内容 【导入】 到当前 mod 关键字的位置
/// 
///                 [声明 mod]
///                             1. 每个 x.rs 文件就是一个 mod
///                             2. 目录下有 mod.rs 则 目录名为一个 mod
///                             3. mod xxx { ... } 则 xxx 为一个 mod 
///                             4. 配合 include! 宏，将某个文件定义为 mod，如：
///                 
///                                     mod toy1 { // 方法1： 使用 include!     // mod toy1
///                                         include!("./toy_implements.rs");
///                                     }
/// 
///                             5. 配合 #[path ="./xxx.rs"] 注解，将某个文件定义为 mod，如：
///                 
///                                     #[path ="./toy_implements.rs"] 
///                                      mod toy2; // 方法2： 使用 path 属性定位文件位置,   mod toy2
/// 
///                             6. 创建与文件夹同名的模块定义文件,也就是将mod.rs改名后提到外层，如：
/// 
///                                     // 目录结构:
///                                     // ┌─main.rs
///                                     // ├─services
///                                     // │  └─user.rs
///                                     // └─services.rs    在和目录同级处定义和目录同名的  rs 文件，即 mod 为目录名
/// 
///                                     // user.rs
///                                     pub fn get(){
///                                         println!("get user");
///                                     }
/// 
///                                     // services.rs
///                                     pub mod user;
/// 
///                                     // main.rs
///                                     mod services;
///                                     fn main() {
///                                         services::user::get();
///                                     }
///                             
///             
///                  [导入 mod]
///                             1. mod xxx；将 mod  xxx 的内容导入到当前 文件中， 位置在  mod xxx；语句处 
/// 
/// 
/// 
///             【pub mod】
/// 
///             pub  mod  xxx ;相当于把 xxx 复制到这个pub mod语句处 再冠以 pub，如：
/// 
///                     // src/toy/cube/mod.rs
///                     pub fn get_size() {
///                         println!("size is in main");
///                         crate::top_size(); // 使用 crate 的 top_size 函数，必不可少的 crate 关键字
///                     }
///                     
///                     // src/toy/mod.rs
///                     pub mod cube; // 将子 mod cube 导入 mod toy 的这个位置，并暴露出去
///                     
///                     // src/main.rs
///                     mod toy;   // 将子 mod toy 导入到这里
///                     fn top_size() {
///                         println!("top size one !")
///                     }
///                     fn main() {
///                         toy::cube::get_size();  // 因为 mod cube 被导入到 mod toy 中，并暴露出来，故可以 toy::cube 这样用
///                     }
/// 
/// 
///             【use】
///                 
///                     调整模块内容调用路径，主要是为了规避 又长又臭的路径  xxx::xxx::xxx
///                     
///                         use 仅仅是在存在模块的前提下，调整调用路径，而没有引入模块的功能，引入模块使用 mod。
///                         
///                         所以 use 必须是在将子 mod 导进来后才可以用， use aa 和 mod aa 需要同时存在当前文件中，方可在当前文件使用 mod aa 的内容
/// 
///             
///             【pub use】
///                         
///                       将子 mod 的内容整合到当前 mod 并对外暴露，让外面看来这些内容是属于当前mod的，如：
/// 
///                         // src/toy/runner.rs
///                         pub fn dog_run() { println!("dog is run !"); }
/// 
///                         // src/toy/fly.rs
///                         pub fn fly_bird() { println!("bird is fly !"); }
///                         
///                         // src/toy/bear.rs
///                         pub fn bear_eat() { println!("bear is eat fish !"); }
///                         pub fn bear_sleep() { println!("bear is go sleep !"); }
///                         
///                         // src/toy/mod.rs
///                         mod runner; // 引入同级 runner.rs 文件
///                         mod fly; // 引入同级 fly.rs 文件
///                         mod bear; // 引入同级 bear.rs 文件
/// 
///                         pub use runner::dog_run; // 声明（导出） dog_run 函数
///                         pub use fly::fly_bird as now_fly_brid; // 声明（导出） fly_bird 函数，并重命名为 now_fly_brid
///                         pub use bear::*; // 声明（导出） dog_run 函数
///                         
///                         // src/main.rs
///                         mod toy;    // 将 mod toy 的内容都导入到这个位置
///                         fn main() {
///                             toy::dog_run();
///                             toy::now_fly_brid();
///                             toy::bear_eat();
///                             toy::bear_sleep();
///                         }
/// 
/// 
/// 
///            【pub(crate) 或者 pub(self) 或者 pub(super) 或者 pub(in <path>)】
/// 
///             1. pub 意味着可见性无任何限制
///             2. pub(crate) 表示在当前包可见
///             3. pub(self) 在当前模块可见
///             4. pub(super) 在父模块可见
///             5. pub(in <path>) 表示在某个路径代表的模块中可见，其中 path 必须是父模块或者祖先模块
/// 
///             如：
/// 
///                             // 一个名为 `my_mod` 的模块
///                             mod my_mod {
/// 
///                                 // 模块中的项默认具有私有的可见性
///                                 fn private_function() {
///                                     println!("called `my_mod::private_function()`");
///                                 }
///                             
///                                 // 使用 `pub` 修饰语来改变默认可见性。
///                                 pub fn function() {
///                                     println!("called `my_mod::function()`");
///                                 }
///                             
///                                 // 在同一模块中，项可以访问其它项，即使它是私有的。
///                                 pub fn indirect_access() {
///                                     print!("called `my_mod::indirect_access()`, that\n> ");
///                                     private_function();
///                                 }
///                             
///                                 // 模块也可以嵌套
///                                 pub mod nested {
/// 
///                                     pub fn function() {
///                                         println!("called `my_mod::nested::function()`");
///                                     }
///                             
///                                     #[allow(dead_code)]
///                                     fn private_function() {
///                                         println!("called `my_mod::nested::private_function()`");
///                                     }
///                             
///                                     // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
///                                     // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
///                                     pub(in crate::my_mod) fn public_function_in_my_mod() {
///                                         print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
///                                         public_function_in_nested()
///                                     }
///                             
///                                     // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
///                                     pub(self) fn public_function_in_nested() {
///                                         println!("called `my_mod::nested::public_function_in_nested");
///                                     }
///                             
///                                     // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
///                                     pub(super) fn public_function_in_super_mod() {
///                                         println!("called my_mod::nested::public_function_in_super_mod");
///                                     }
///                                 }
///                             
///                                 pub fn call_public_function_in_my_mod() {
///                                     print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
///                                     nested::public_function_in_my_mod();
///                                     print!("> ");
///                                     nested::public_function_in_super_mod();
///                                 }
///                             
///                                 // `pub(crate)` 使得函数只在当前包中可见
///                                 pub(crate) fn public_function_in_crate() {
///                                     println!("called `my_mod::public_function_in_crate()");
///                                 }
///                             
///                                 // 嵌套模块的可见性遵循相同的规则
///                                 mod private_nested {
///                                     #[allow(dead_code)]
///                                     pub fn function() {
///                                         println!("called `my_mod::private_nested::function()`");
///                                     }
///                                 }
///                             }
///                             
///                             fn function() {
///                                 println!("called `function()`");
///                             }
///                             
///                             fn main() {
///                                 // 模块机制消除了相同名字的项之间的歧义。
///                                 function();
///                                 my_mod::function();
///                             
///                                 // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
///                                 my_mod::indirect_access();
///                                 my_mod::nested::function();
///                                 my_mod::call_public_function_in_my_mod();
///                             
///                                 // pub(crate) 项可以在同一个 crate 中的任何地方访问
///                                 my_mod::public_function_in_crate();
///                             
///                                 // pub(in path) 项只能在指定的模块中访问
///                                 // 报错！函数 `public_function_in_my_mod` 是私有的
///                                 //my_mod::nested::public_function_in_my_mod();
///                                 // 试一试 ^ 取消该行的注释
///                             
///                                 // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的
///                             
///                                 // 报错！`private_function` 是私有的
///                                 //my_mod::private_function();
///                                 // 试一试 ^ 取消此行注释
///                             
///                                 // 报错！`private_function` 是私有的
///                                 //my_mod::nested::private_function();
///                                 // 试一试 ^ 取消此行的注释
///                             
///                                 // 报错！ `private_nested` 是私有的
///                                 //my_mod::private_nested::function();
///                                 // 试一试 ^ 取消此行的注释
///                             }
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 