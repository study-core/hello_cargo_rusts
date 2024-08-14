/// -Z trace-macros
/// 
/// ######################################################################################################################################################
/// 宏  的 定义 及 使用
/// 
/// rust 的宏基本是  部分卫生性的
/// ###################################################################################################################################################### 
/// 
/// 1、 声明宏：   编写类似于 match 表达式
/// 2、 过程宏：   对它所给的 Rust 代码的抽象语法树（AST）进行操作， 如: #[derive(Debug)]
/// 
/// 
/// 
/// 宏展开：  rustc -Z unpretty=expanded ./src/main.rs  
/// 
/// 宏调试:   rustc -Z trace-macros ./src/main.rs  需要配合 #![feature(trace_macros)] 用
/// 
/// 
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// 
///                                                                 声明宏 的 定义 及 使用
/// 
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// ###################################################################################################################################################### 
/// 
/// 
/// 
/// 
/// 
/// 宏定义的内容由 规则(rule) 组成, 每一条规则都形如:
///
///  (Pattern) => { Exapnsion };
/// 
/// 其中所使用的的 括号  (), [], {} 是不受限制的, 调用时不会对括号进行检查.
/// 
/// () => {} 看起来很神秘, 因为它不是标准的 rust 语法, 是 macro_rules! 这个宏自己发明的, 用来表示一条宏规则:   "=>" 左边是【匹配模式】, 右边是【等待展开的代码】
/// 
/// 类似于Rust中的match语句, 在macro_rules!中可以定义多条宏规则,  当宏被调用时, 会由上而下对每个规则进行匹配, 如果某一条规则与输入 完全 匹配, 则立刻进行该规则所对应的展开.
/// 
///
/// 
/// macro_rules! hey{
///     () => {}, 
///     () => {}
/// }
/// 
/// ----------------------------------------------------------------------------------------------
/// 
/// 又如:
/// 
/// 
/// macro_rules! add{
///     // first arm match add!(1, 2),  add!(2, 3) etc
///        ($a:expr, $b:expr)=>{
///            {
///                $a+$b
///            }
///        };
///    // Second arm macth add!(1),  add!(2) etc
///        ($a:expr)=>{
///            {
///                $a
///            }
///        }
///    }
///    
///    fn main(){
/// 
///        let x=0;
/// 
///        add!(1, 2);   // 匹配到 宏中 第一个规则
///        add!(x);     // 匹配到 宏中 第二个规则
///    }
/// 
/// 
/// ----------------------------------------------------------------------------------------------
/// 
/// 宏的参数描述为:  
/// 
///         **************************
///         *   $参数名称: 参数类型   *
///         **************************
/// 
/// 如:         $a:expr
/// 
/// 参数类型叫做: 选择器、token类型
/// 
/// 
/// 
/// 
/// 【参数类型】
/// 
/// 
/// 
///         1、 item:           Item, 如：函数定义, 常量声明, 结构体, 模块, impl 块 等等
///         2、 block:          BlockExpression, 一个语句块 或 一个表达式, 由花括号所包围, 如: { ... }
///         3、 stmt:           Statement, 语句, 如: let 表达式 (传入为 stmt 类型的参数时 【不需要末尾的分号】, 但需要分号的 item 语句除外)
///         4、 pat:            Pattern, 模式匹配中的 【模式】, 如: Some(a)
///         5、 expr:           Expression, 表达式, 如: Vec::new()
///         6、 ty:             Type, 类型, 如: i32
///         7、 ident:          IDENTIFIER_OR_KEYWORD, 【标识符】 或 【关键字】, 如: i 或 self
///         8、 path:           TypePath, 类型路径, 如: std::result::Result
///         9、 tt:             TokenTree, Token 树 (单棵标记树), 被匹配的定界符 (、[] 或 {} 中的单个或多个 token  【理论上可以匹配所有 ...】
///         10、 meta:          Attr, 形如: #[...] 和 #![...] 的属性内部的内容
///         11、 lifetime:      LIFETIME_TOKEN, 生命周期 Token, 如: 'foo、'static
///         12、 vis:           Visibility, 可能为空的 【可见性限定符】, 如: pub、pub(in crate)
///         13、 literal:       LiteralExpression, 一个字面值, 如: "Hello World!"、3.14、'🦀'
/// 
///     【其中, tt 类型可以被视为 Rust 宏的 Any】
///      
///     【有一个特殊的元变量叫做 $crate ，它用来指代当前 crate】
/// 
/// 
/// 宏还对 各种类型的参数  捕获之后  所允许的内容  添加了限制, 以避免语义冲突:
/// 
/// 
/// 
///         1、 item: 任何标记
///         2、 block: 任何标记
///         3、 stmt: =>、;、,
///         4、 expr: =>、;、,
///         5、 pat: =>、,、=、|、if 或 in
///         6、 ty: {、[、=>、,、>、=、:、;、|、as 或 where
///         7、 ident: 任何标记
///         8、 path: {、[、=>、,、>、=、:、;、|、as 或 where
///         9、 meta: 任何标记
///         10、 tt: 任何标记
/// 
/// 
/// 
/// 【重复模式】
/// 
/// 
///         *****************************************************************************
///         *       $( 需要重复的内容 )      [可选的]分隔符          [必选的]重复标志   *
///         *                                                                           *
///         *  即： $( ... )                  sep                        rep            *
///         *****************************************************************************
/// 
/// sep    可选的分隔符:        常见的有 , 和 ;
/// 
/// rep    必选的重复标记:      三种     * (0 到 n 次)       + ( 1 到 n次)        ? (最多一次  ，所以此时不能前跟分隔标记)
/// 
/// 如:
///         $( $i:expr ),
///         $( $i:expr, )*
///         $( $i:expr ),* $(,)?
/// 
/// 
/// 如下:
/// 
/// 
///          入零个或多个相同类型的值, 构造一个包含这些值（按照顺序）的 Vec.
///        
///          macro_rules! build_vec {
///              (
///                  $( $i:expr ),*                          // 重复, 以支持任意数量的参数
///                  $(,)?                                   // 可选的末尾逗号
///              ) => {
///                  {                                       // 创建一个块, 以支持多条语句
///                      let mut vec = Vec::new();           // 构造一个 Vec, 必须为 mut, 否则下文无法进行 push
///          
///                      $(                                  // 重复, 将每个 $i 推入 vec 中
///                          vec.push($i);
///                      )*
///          
///                      vec                                 // 返回 vec
///                  }
///              }
///          }
///        
/// ----------------------------------------------------------------------------------------------
/// 
/// 又如:
/// 
/// 
///          macro_rules! add{
///  
///              // 单参数情况下
///                 ($a:expr)=>{
///                     $a
///                 };
///  
///             // 在传递两个参数的情况下
///                 ($a:expr,$b:expr)=>{
///                     {
///                         $a+$b
///                     }
///                 };
///  
///             // 使用其他参数再次调用 add 宏
///                 ($a:expr,$($b:tt)*)=>{
///                    {
///                        $a+add!($($b)*)
///                    }
///                 }
///             }
///             
///             fn main(){
///               println!("{}",add!(1,2,3,4));
///             }
/// 
/// 
/// ----------------------------------------------------------------------------------------------
/// 
/// 又如:
/// 
/// 
/// 
/// 
///             macro_rules! make_public{
///                 (
///                  $(#[$meta:meta])*
///                  $vis:vis struct $struct_name:ident {
///                     $(
///                     $(#[$field_meta:meta])*
///                     $field_vis:vis $field_name:ident : $field_type:ty
///                     ),*$(,)+
///                 }
///                 ) => {
///             
///                         $(#[$meta])*
///                         pub struct $struct_name{
///                             $(
///                             $(#[$field_meta:meta])*
///                             pub $field_name : $field_type,
///                             )*
///                         }
///                 }
///             }
///             
///             fn main(){
///                 make_public!{
///                     #[derive(Debug)]
///                     struct Name{
///                         n:i64,
///                         t:i64,
///                         g:i64,
///                     }
///                 }
///             
///                 // 得到：
///                 // #[derive(Debug)]
///                 // pub struct name {
///                 //     pub n: i64,
///                 //     pub t: i64,
///                 //     pub g: i64,
///                 // }
///             }
///             
///             
/// 
/// ----------------------------------------------------------------------------------------------
/// 
/// 
/// 
/// 【元变量表达式】
/// 
/// 
/// Rust 有一些特殊的元变量表达式（以下简称表达式）：transcriber (转码器) 可以使用这些表达式来获取有关元变量的信息
/// 
/// 
///             1. $$：                         展开为单个 $，这会有效地转义 $ 标记，因此它不会被展开 (转码)
///             2. ${count($ident)}：           【最里】层 $ident 总共重复的次数，相当于 ${count(ident, 0)}
///             3. ${count($ident，depth)}：    第 depth 层 $ident 总共重复的次数
///             4. ${index()}：                 【最里】层重复的当前重复的索引，相当于 ${index(0)}
///             5. ${index(depth)}：            在第 depth 层处当前重复的索引，向外计数
///             6. ${len()}：                   【最里】层重复的【重复次数】，相当于 ${length(0)}
///             7. ${len(depth)}：              在第 depth 层重复的次数，向外计数
///             8. ${ignore(ident)}：           绑定 $ident 进行重复，并展开成空
///         
/// 
/// 
///     1. 【$$】
/// 
///     错误示例: (不用 $$ 时)
/// 
///             macro_rules! foo {
///                 () => {
///                     macro_rules! bar {
///                         ( $( $any:tt )* ) => { $( $any )* };    
///                         // ^^^^^^^^^^^ error: attempted to repeat an expression containing no syntax variables matched as repeating at this depth
///                         //
///                         // foo! 的 transcriber 看到有重复捕获的意图，并试图重复捕获，但它的作用域中没有 $any 元变量，这导致它产生错误.
///                         // 即在 foo 中没有 $any 元变量，则在 bar 中使用，则错误
///                     }
///                 };
///             }
///             
///             fn main() {
///                 foo!();
///             }
/// 
/// 
/// 
///     正确示例:  (使用 $$ 时)
/// 
///             #![feature(macro_metavar_expr)]
///             
///             macro_rules! foo {
///                 () => {
///                     macro_rules! bar {
///                         ( $$( $$any:tt )* ) => { $$( $$any )* };
///                     }
///                 };
///             }
///             
///             fn main() {
///                 foo!();
///                 bar!();
///             }
/// 
/// 
/// 
///         
///     2. 【${count($ident)}】 和 【${count($ident，depth)}】
/// 
///         
/// 
///         count 表达式展开成元变量 $ident 在给定重复深度的重复次数。
/// 
///         
///         
///                 $ident 参数必须是规则作用域中声明的元变量
///                 depth 参数必须是值 <= 元变量 $ident 出现的最大重复深度的整型字面值
///                 count($ident, depth) 展开成不带后缀的整型字面值标记
///                 count($ident) 是 count($ident, 0) 的简写; 即： ${count($ident)} 相当于 ${count($ident，0)}
/// 
/// 
///         例如:  
///         
///                 ${count(x)} $( $x )*  
///         
///         表达式 ${count(x)} 将扩展为无后缀的整数文字，等于 $( $x )* 重复的重复次数。例如，如果元变量 $x 重复四次，那么它将扩展到整数文字 4
/// 
///         如果重复是嵌套的，则可以使用可选的深度参数来限制计算的嵌套重复的数量。例如，宏扩展如下：
/// 
///                 ${count(x, 1)} ${count(x, 2)} ${count(x, 3)} $( a $( b $( $x )* )* )*
/// 
///         扩展为的三个值是最外层重复次数（ a 生成的次数）、中间重复次数的总和（ b 生成的次数）以及 $x 的重复总数
/// 
///     
/// 
///             #![feature(macro_metavar_expr)]
///             
///             macro_rules! foo {
///                 ( $( $outer:ident ( $( $inner:ident ),* ) ; )* ) => {
///                     println!("count(outer, 0): $outer repeats {} times", ${count($outer)});                                             // 4
///                     println!("count(inner, 0): The $inner repetition repeats {} times in the outer repetition", ${count($inner, 0)});   // 3
///                     println!("count(inner, 1): $inner repeats {} times in the inner repetitions", ${count($inner, 1)});                 // 4
///                 };
///             }
///             
///             fn main() {
///                 foo! {
///                     outer () ;
///                     outer ( inner , inner ) ;
///                     outer () ;
///                     outer ( inner ) ;
///                 };
///             }
/// 
/// 
/// 
///     3. 【${index()}】和【${index(depth)}】
/// 
///         index(depth) 表达式展开为给定重复深度下，当前的迭代索引。
///             
///             depth 参数表明在第几层重复，这个数字  从最内层重复 调用表达式开始 向外 计算
///             index(depth) 展开成不带后缀的整型字面值标记
///             index() 是 index(0) 的简写
/// 
/// 
/// 
/// 
/// 
/// 
///             #![feature(macro_metavar_expr)]
///             //
///             // (("hello", 0, 0), ("indices", 1, 0), ("of", 1, 1), ("these", 3, 0), ("repetitions", 3, 1))
///             //
///             macro_rules! attach_iteration_counts {
///                 ( $( ( $( $inner:ident ),* ) ; )* ) => {
///                     ( $(
///                         $((
///                             stringify!($inner),
///                             ${index(1)},        // 这指的是外层重复
///                             ${index()}          // 这指的是内层重复，等价于 `index(0)`
///                         ),)*
///                     )* )
///                 };
///             }
///             
///             fn main() {
///                 let v = attach_iteration_counts! {
///                     ( hello ) ;                     // hello 在 ${index(1)} 层索引为 0, 在 ${index()} 层的索引为 0
///                     ( indices , of ) ;              // indices 在 ${index(1)} 层索引为 1, 在 ${index()} 层的索引为 0 ； of 在 ${index(1)} 层索引为 1, 在 ${index()} 层的索引为 1
///                     () ;                            // 没有元素 $inner ，故不打印
///                     ( these, repetitions ) ;        // these 在 ${index(1)} 层索引为 3, 在 ${index()} 层的索引为 0 ; repetitions 在 ${index(1)} 层索引为 3, 在 ${index()} 层的索引为 1
///                 };
///                 println!("{v:?}");
///             }
/// 
/// 
/// 
///             小知识: 
/// 
///                     stringify! 是 Rust 提供的内置宏，可以将一个表达式(例如 1 + 2) 在 【编译期】 转换成一个字符串字面值("1 + 2")，
///                     该字面量会直接打包进编译出的二进制文件中，具有 'static 生命周期。
///                     
///                     使用 stringify! 有两个好处:
///                     
///                                             1. 入参可能是一个表达式，我们需要它的字面值形式
/// 
///                                             2. 可以减少一次 String 带来的内存分配 (在编译器编译字面量到二进制中了 ...)
/// 
/// 
/// 
/// 
/// 
/// 
/// 
///     4. 【${len()}】 和 【${len(depth)}】
/// 
///         length(depth) 表达式展开为在给定重复深度的迭代次数。
///             
///             depth 参数表示在第几层重复，这个数字从最内层重复调用表达式开始向外计算
///             length(depth) 展开成不带后缀的整型字面值标记
///             length() 是 length(0) 的简写
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
///             #![feature(macro_metavar_expr)]
///             //
///             // 'small' in inner iteration 0/2 with 'many' in outer iteration 0/3 
///             // 'things' in inner iteration 1/2 with 'many' in outer iteration 0/3 
///             // 'one' in inner iteration 0/1 with 'exactly' in outer iteration 2/3 
///             //
///             macro_rules! lets_count {
///                 ( $( $outer:ident ( $( $inner:ident ),* ) ; )* ) => {
///                     $(
///                         $(
///                             println!(
///                                 "'{}' in inner iteration {}/{} with '{}' in outer iteration {}/{} ",
///                                 stringify!($inner), ${index()}, ${len()},
///                                 stringify!($outer), ${index(1)}, ${len(1)},
///                             );
///                         )*
///                     )*
///                 };
///             }
///             
///             fn main() {
///                 lets_count!(
/// 
///                     // small 在 ${index()} 层索引为 0, 而当前里层元组的长度 ${len()} 为 2; things 在 ${index()} 层索引为 1, 而当前里层元组的长度 ${len()} 为 2;
///                     // many  在 ${index(1)} 层索引为 0，而当前外层元组的长度 ${len(1) 为 3
///                     many (small , things) ; 
/// 
///                     // 由于没有里层元组，则直接不打印了 ...    
///                     none () ;
/// 
///                     // one 在 ${index()} 层索引为 0, 而当前里层元组的长度 ${len()} 为 1
///                     // exactly  在 ${index(1)} 层索引为 2，而当前外层元组的长度 ${len(1) 为 3
///                     exactly ( one ) ;
///                 );
///             }
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
///     5. 【${ignore(ident)}】
/// 
///         ignore(ident) 表达式展开为空，这使得在无需实际展开元变量的时候，像元变量重复展开相同次数的某些内容
///             
///             ident 参数必须是规则作用域中声明的元变量
/// 
/// 
/// 
/// 
/// 
/// 
/// 
///             #![feature(macro_metavar_expr)]
///             //
///             // ((0, 0), (1, 0), (0, 2), (0, 3), (1, 3), (2, 3))
///             //
///             macro_rules! repetition_tuples {
///                 ( $( ( $( $inner:ident ),* ) ; )* ) => {
///                     ($(
///                         $(
///                             (
///                                 ${index()},
///                                 ${index(1)}
///                                 ${ignore($inner)} // without this metavariable expression, compilation would fail    如果没有这个元变量表达式，编译就会失败
///                                 
///                                 // 上述和以下类似:
///                                 //
///                                 //  stringify!($inner),
///                                 //  ${index(1)},        // 这指的是外层重复
///                                 //  ${index()}          // 这指的是内层重复，等价于 `index(0)`
///                                 //
///                                 // 只不过人家 stringify!($inner) 是字符拼接 $inner ，而 ${ignore($inner)} 是忽略掉 $inner
///                             ),
///                         )*
///                     )*)
///                 };
///             }
///             
///             fn main() {
///                 let tuple = repetition_tuples!(
///                     
///                     // one 在 ${index()} 层索引为 0, 在 ${index(1)} 层的索引为 0, ignore 掉 $inner 故: (0, 0) 
///                     // two 在 ${index()} 层索引为 1, 在 ${index(1)} 层的索引为 0, ignore 掉 $inner 故: (1, 0)
///                     ( one, two ) ;   
///                     
///                     // 没有元素 $inner ，故不打印             
///                     () ;
///                     
///                     // one 在 ${index()} 层索引为 0, 在 ${index(1)} 层的索引为 2, ignore 掉 $inner 故: (0, 2) 
///                     ( one ) ;
///                     
///                     // one 在 ${index()} 层索引为 0, 在 ${index(1)} 层的索引为 3, ignore 掉 $inner 故: (0, 3) 
///                     // one 在 ${index()} 层索引为 1, 在 ${index(1)} 层的索引为 3, ignore 掉 $inner 故: (1, 3) 
///                     // one 在 ${index()} 层索引为 2, 在 ${index(1)} 层的索引为 3, ignore 掉 $inner 故: (2, 3) 
///                     ( one, two, three ) ;
///                 );
///                 println!("{tuple:?}");
///             }
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 【导入、导出宏】
/// 
///          前面提到宏名是按顺序解析的，所以从其它模块中导入宏时与导入函数、trait 的方式不太一样，宏导入导出用 #[macro_use] 和 #[macro_export]
/// 
/// 
/// 
/// #[macro_use]
/// 
///             
///         1、导出该模块内的所有宏， 从而让导出的宏在所定义的模块结束之后依然可用
/// 
///
///             #![allow(unused)]
///             
///             #[macro_use]   // 加了这行，使之宏 m 在 mod inner 之外还有意义
///             mod inner {
///                 macro_rules! m {
///                     () => {};
///                 }
///             }
///             
///             fn main() {
///                 m!();
///             }
///         
/// 
///         2、用于从另一个 crate 里来导入宏，方法是将它附加到当前 crate 根模块中的 extern crate 声明前
/// 
///             (要用 #[macro_use] 导入宏，必须先使用 #[macro_export] 将被使用的宏导出)
/// 
///             #[macro_use(lazy_static)] // 或者使用 #[macro_use] 来导入所有宏.
///             extern crate lazy_static;
///             
///             lazy_static!{}
///             // self::lazy_static!{} // 报错: lazy_static 没在 `self` 中定义
/// 
/// 
/// 
/// 
/// 
/// 
/// #[macro_export]
/// 
///             【默认情况下，宏没有基于路径的作用域。】
///             
///             crate 之间只有被标为 #[macro_export] 的宏可以被其它 crate 导入。
/// 
///             宏带有 #[macro_export] 属性，则相当于它在  当前 crate 的  根作用域  的顶部被声明
/// 
///             (标有 #[macro_export] 的宏始终是 pub 的，以便可以通过  路径 (使用 use 类似引用其他对象一样引用) 或前面  所述的 #[macro_use] 方式让其他 crate 来引用)
/// 
/// 
/// 
///             #![allow(unused)]
///             
///             mod inner {
///                 super::m!();    // 由于 mod mac 将宏 m 导出到当前 crate 的顶部，(又 mod inner 的 super 是当前 main mod)，可以这样用 
///                 crate::m!();    // 由于 mod mac 将宏 m 导出到当前 crate 的顶部，可以这样用 
///             }
///             
///             mod mac {
///                 #[macro_export]     // 导出 宏 m 到当前  crate 的顶部声明
///                 macro_rules! m {
///                     () => {};
///                 }
///             }
///             
///             
///             fn main() {
///                 self::m!(); // 由于 mod mac 将宏 m 导出到当前 crate 的顶部，可以这样用 
///                 m!();       // OK: 基于路径的查找发现 m 在当前模块中有声明.
///             }
/// 
/// 
/// 
/// 
/// 
/// 【宏的  卫生性】    使用  $crate 查找宏
/// 
/// 
///     默认情况下： 
///                 宏中引用的所有标识符都按原样展开，并在宏的调用位置上去查找。
/// 
///     异常情况下：
///                 如果 宏引用 的 程序项 或 宏不在调用位置的作用域内，则这可能会导致问题。
/// 
/// 
///             // 在 `helper_macro` crate 中.
///             #[macro_export]
///             macro_rules! helped {
///                 // () => { helper!() } // 这可能会导致错误，因为 'helper' 在当前作用域之后才定义.
///                 () => { $crate::helper!() }
///             }
///             
///             #[macro_export]
///             macro_rules! helper {
///                 () => { () }
///             }
///             
///             // 在另一个 crate 中使用.
///             // 注意没有导入 `helper_macro::helper`!
///             use helper_macro::helped;
///             
///             fn unit() {
///                 helped!();
///             }
///             
/// 
/// 
///     【由于 $crate 指的是当前的（$crate 源码出现的）crate，因此在引用非宏程序项时，它必须与全限定模块路径一起使用】
///                 
///         
/// 
/// 
/// 
///             #![allow(unused)]
///             
///             
///             pub mod inner {
///             
///                 #[macro_export]                         // 导出宏 call_foo
///                 macro_rules! call_foo {
///             
///                     () => { $crate::inner::foo() };     // 从当前 crate  开始引用 mod inner 的 foo 函数
///             
///                 }
///             
///                 pub fn foo() {
///                     println!("hello foo");
///                 }
///             }
///             
///             #[macro_use(inner::call_foo)]               // 引用 mod inner 导出来的宏 call_foo
///             
///             fn main() {
///                 call_foo!();
///             }
/// 
/// 
/// 
/// 
/// 
///     【$crate 引用的 程序项 或者  宏， 必须在调用位置处可见， 即： $crate 受可见性条件约束】
/// 
/// 
///             
///             #![allow(unused)]
///             
///             
///             mod inner {
///                 #[macro_export]
///                 macro_rules! call_foo {
///                     () => { $crate::foo() };  // 因为 inner::foo 函数是 private 的，对于  crate 来说是不可以直接访问到的
///                 }
///                 
///                 fn foo() {
///                     println!("hello foo");
///                 }
///             }
///             
///             
///             fn main() {
///                 call_foo!();
///             }
///              
/// 
/// 
/// 
///     【当一个宏被导出时，可以在 #[macro_export] 属性里添加 local_inner_macros 属性值，用以自动为该属性修饰的宏内包含的所有宏调用自动添加 $crate:: 前缀】
/// 
/// 
/// 
/// 
/// 
///             #![allow(unused)]
///             
///             #[macro_export(local_inner_macros)]
///             macro_rules! helped {
///                 () => { helper!() } // 自动转码为 $crate::helper!().
///             }
///             
///             #[macro_export]
///             macro_rules! helper {
///                 () => { () ; println!("hello helper");}
///             }
///             
///             fn main() {
///                 helped!();
///             }
///             
/// 
/// 
/// 
/// 
/// 【示例】 使用 卫生宏 实现 斐波那契数列
/// 
/// 
/// 
/// 
/// 
///             #[macro_export] // 导出
///             macro_rules! count_exprs {
///                 () => (0);
///                 ($head:expr) => (1);
///                 ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
///             }
///             
///             #[macro_export] // 导出
///             macro_rules! recurrence {
///                 ( $seq:ident [ $ind:ident ]: $sty:ty = $($inits:expr),+ ; ... ; $recur:expr ) => {
///             //    ^~~~~~~~~~   ^~~~~~~~~~ changed
///                     {
///                         use std::ops::Index;
///                         use $crate::count_exprs; // 导入
///             
///                         const MEM_SIZE: usize = count_exprs!($($inits),+);
///             
///                         struct Recurrence {
///                             mem: [$sty; MEM_SIZE],
///                             pos: usize,
///                         }
///             
///                         struct IndexOffset<'a> {
///                             slice: &'a [$sty; MEM_SIZE],
///                             offset: usize,
///                         }
///             
///                         impl<'a> Index<usize> for IndexOffset<'a> {
///                             type Output = $sty;
///             
///                             #[inline(always)]
///                             fn index<'b>(&'b self, index: usize) -> &'b $sty {
///                                 use std::num::Wrapping;
///             
///                                 let index = Wrapping(index);
///                                 let offset = Wrapping(self.offset);
///                                 let window = Wrapping(MEM_SIZE);
///             
///                                 let real_index = index - offset + window;
///                                 &self.slice[real_index.0]
///                             }
///                         }
///             
///                         impl Iterator for Recurrence {
///                             type Item = $sty;
///             
///                             #[inline]
///                             fn next(&mut self) -> Option<$sty> {
///                                 if self.pos < MEM_SIZE {
///                                     let next_val = self.mem[self.pos];
///                                     self.pos += 1;
///                                     Some(next_val)
///                                 } else {
///                                     let next_val = {
///                                         let $ind = self.pos;
///             //                              ^~~~ changed
///                                         let $seq = IndexOffset { slice: &self.mem, offset: $ind };
///             //                              ^~~~ changed
///                                         $recur
///                                     };
///             
///                                     {
///                                         use std::mem::swap;
///             
///                                         let mut swap_tmp = next_val;
///                                         for i in (0..MEM_SIZE).rev() {
///                                             swap(&mut swap_tmp, &mut self.mem[i]);
///                                         }
///                                     }
///             
///                                     self.pos += 1;
///                                     Some(next_val)
///                                 }
///                             }
///                         }
///             
///                         Recurrence { mem: [$($inits),+], pos: 0 }
///                     }
///                 };
///             }
///             
///             fn main() {
///                 // let fib = recurrence![a[n]: u64 = 0, 1; ...; a[n-1] + a[n-2]];
///                 // 
///                 // for e in fib.take(10) { println!("{}", e) }
///             
///                 for e in recurrence!(f[i]: f64 = 1.0; ...; f[i-1] * i as f64).take(10) {
///                     println!("{}", e)
///                 }
///                 
///             }
///             
///             
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// --------------------------------------- trace 调试宏 -------------------------------------------------
///             
///             #![feature(trace_macros)]
///             
///             macro_rules! each_tt {
///                 () => {};
///                 ($_tt:tt $($rest:tt)*) => {each_tt!($($rest)*);};
///             }
///             
///             fn main () {
///                 each_tt!(foo bar baz quux);
///                 trace_macros!(true);
///                 each_tt!(spim wak plee whum);
///                 trace_macros!(false);
///                 each_tt!(trom qlip winp xod);
///             
///             }
///             
/// --------------------------------------- log 调试宏 -------------------------------------------------
///             
///             #![feature(log_syntax)]
///             
///             macro_rules! sing {
///                 () => {};
///                 ($tt:tt $($rest:tt)*) => {log_syntax!($tt); sing!($($rest)*);};
///             }
///             
///             fn main () {
///                 sing! {
///                     ^ < @ < . @ *
///                     '\x08' '{' '"' _ # ' '
///                     - @ '$' && / _ %
///                     ! ( '\t' @ | = >
///                     ; '\x08' '\'' + '$' ? '\x7f'
///                     , # '"' ~ | ) '\x07'
///                 }
///             }
///             
///             
/// --------------------------------------- tt 撕咬机 -------------------------------------------------
///             
///             macro_rules! mixed_rules {
///             
///                 // a 模式
///                 () => {};
///             
///                 // b 模式
///                 (trace $name:ident; $($tail:tt)*) => {
///                     {
///                         // $name 在 println! 中，需要被捕获成 string，如： println!(" = {:?}", $name) 看下就知道了
///                         println!(concat!(stringify!($name), " = {:?}"), $name);         
///                         mixed_rules!($($tail)*);
///                     }
///                 };
///             
///                 // c 模式
///                 (trace $name:ident = $init:expr; $($tail:tt)*) => {
///                     {
///                         let $name = $init;                                          
///                         println!(concat!(stringify!($name), " = {:?}"), $name);
///                         mixed_rules!($($tail)*);
///                     }
///                 };
///             }
///             
///             fn main() {
///                 let a = 42;                                 
///                 let b = "Ho-dee-oh-di-oh-di-oh!";                      
///                 let c = (false, 2, 'c');      
///                 mixed_rules!(
///                     trace a;                                                    // 匹配 b 模式      a = 42                                    
///                     trace b;                                                    // 匹配 b 模式      b = "Ho-dee-oh-di-oh-di-oh!"
///                     trace c;                                                    // 匹配 b 模式      c = (false, 2, 'c')
///                     trace b = "They took her where they put the crazies.";      // 匹配 c 模式      b = "They took her where they put the crazies."
///                     trace b;                                                    // 匹配 b 模式      b = "They took her where they put the crazies."    这里是因为上面一次 b 被赋值了
///                 );
///             }
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 【声明宏的缺点】
/// 
///             1、缺少对宏的自动完成和展开的支持
///             
///             2、声明式宏调式困难
///             
///             3、修改能力有限
///             
///             4、更大的二进制
///             
///             5、更长的编译时间 (这一条对于【声明宏】和【过程宏】都存在)
/// 
/// 
/// 
/// 
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// 
///                                                             过程宏 的 定义 及 使用
/// 
/// ###################################################################################################################################################### 
/// ######################################################################################################################################################
/// ######################################################################################################################################################
/// 
/// 
/// 与声明宏不同，过程宏采用 Rust 函数的形式，接受一个（或两个）标记流 TokenStream 并输出一个标记流 TokenStream
/// 
/// 
/// 过程宏的核心：
///         
///         只是一个从 proc-macro crate type 这种类型的库中所导出的公有函数，因此当编写多个过程宏时，你可以将它们全部放在一个 crate 中。
/// 
/// 
/// 在使用 Cargo 时，定义一个 proc-macro crate 的方式是将 Cargo.toml 中的 lib.proc-macro 键设置为 true，就像这样  (例如： hello_procedural_macros 的 Cargo.toml 中添加)
///                     
///                     // Cargo.toml
///                     [lib]
///                     proc-macro = true
/// 
/// 
/// 
/// proc-macro 类型的 crate 会隐式链接到编译器提供的 proc_macro 库， proc_macro 库包含了开发过程宏所需的所有内容，并且它公开了两个最重要的类型：
///             
///             1. TokenStream:     它表示我们所熟知的标记树
///             2. Span:            它表示源代码的一部分，主要用于错误信息的报告和卫生性
/// 
/// 
///  
/// 
/// (过程宏其实是在 *token流(token streams)*上操作，而不是在某个或某些 AST 节点上操作。token流大致相当于 Vec<TokenTree>，其中 TokenTree 可以大致视为词法 token。
/// 例如，foo 是标识符(Ident)类型的 token，. 是一个标点符号(Punct)类型的 token，1.2 是一个字面量(Literal)类型的 token。不同于 Vec<TokenTree> 的是 TokenStream 的克隆成本很低。
/// 所有类型的 token 都有一个与之关联的 Span。
/// Span 是一个不透明的值，不能被修改，但可以被制造。Span 表示程序内的源代码范围，主要用于错误报告。可以事先（通过函数 set_span）配置任何 token 的 Span。)
/// 
/// 
/// 
/// 过程宏是存在于 crate 中的函数，所以它们可以像 Rust 项目中的所有其他条目一样使用
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
///         1. 类函数  过程宏 (Function-like macros)
///         3. 类属性  过程宏 (Attribute-like macros)
///         2. 派生    过程宏 (Derive macros)
///         
/// 
/// 
/// 
/// 
/// 
/// 【1.【类函数】 过程宏】
/// 
/// 
/// 类函数过程宏 是由一个带有 proc_macro属性和 (TokenStream) -> TokenStream签名的 公有可见性函数定义
/// 
/// 类函数过程宏 像声明宏那样被调用，即 makro!(…) 这样调用；它也是唯一一个在单独看调用形式时，无法与声明宏区分开的宏
/// 
/// 
/// 
/// (与声明性宏不同，类函数过程宏 对其输入没有特定的限制
/// 
/// 即不支持 类型参数，因为 【过程宏 直接作用于标记】，而不是根据片段分类符 类型参数或类似的东西（比如反复）匹配它们
/// 
//  过程宏更强大，因为它们可以任意修改其输入，并生成任何所需的输出，只要输出在 Rust 的语法范围内)
/// 
/// 
/// 类函数过程宏 可以在任何宏调用位置调用：
///         
///         这些位置包括语句、表达式、模式、类型表达式、程序项可以出现的位置 (包括extern块里、固有(inherent)实现里和 trait实现里、以及 trait声明里)
/// 
/// 
/// 
/// 
/// 
/// 
/// 如, 在 hello_procedural_macros 的 Cargo.toml 中添加:  
/// 
///             [lib]
///             proc-macro = true
/// 
/// 然后在 hello_procedural_macros 中定义:  
/// 
/// 
/// 
///             // 定义 【类函数】 过程宏
///             #![crate_type = "proc-macro"]
///             extern crate proc_macro;
///             use proc_macro::TokenStream;
///             
///             #[proc_macro]
///             pub fn make_answer(_item: TokenStream) -> TokenStream {
///                 "fn answer() -> u32 { 42 }".parse().unwrap()
///             }
/// 
/// 最后在项目中使用:
/// 
/// 
/// 
/// 
/// 
/// 
///             // 使用 hello_procedural_macros 中定义的 【类函数】 过程宏
///             extern crate hello_procedural_macros;
///             use hello_procedural_macros::make_answer;
///             
///             
///             // 调用 宏
///             make_answer!(Gavin);                        // 输出 Gavin, 并声明了 fn answer 函数
/// 
///             // 注意: 也可以不输入任何直接调用, 像这样  make_answer!();  // 只声明了 fn answer 函数
/// 
///             fn main() {
///                 // 使用 宏被调用后结果生成的 函数
///                 println!("{}", answer());               // 输出 42 
///             }
///             
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 【2.【类属性】 过程宏】
/// 
/// 
/// 
/// 
/// 
/// 类属性过程宏 定义可以附加到程序项上的新的外部属性，这些程序项包括外部(extern)块、固有实现、trate实现，以及 trait声明中的各类程序项
/// 
/// 类属性过程宏 定义了可添加到条目的的新外部属性。这种宏通过 #[attr] 或 #[attr(…)] 方式调用
/// 
/// 
/// 类属性过程宏 有两个输入参数：
///             
///             第一个参数是 属性名称后面的带分隔符的标记树，不包括它周围的分隔符。   如果只有属性名称（其后不带标记树，比如 #[attr]），则这个参数的值为空
/// 
///             第二个参数是 添加了该过程宏属性的条目，但不包括该过程宏所定义的属性。 因为这是一个 active 属性，在传递给过程宏之前，该属性将从条目中剥离出来
/// 
/// 
/// 
/// 
/// 
/// 如, 在 hello_procedural_macros 的 Cargo.toml 中添加:  
/// 
///             [lib]
///             proc-macro = true
/// 
/// 然后在 hello_procedural_macros 中定义:  
/// 
/// 
/// 
///             // 定义 【类属性】 过程宏
///             #![crate_type = "proc-macro"]
///             extern crate proc_macro;
///             use proc_macro::TokenStream;
/// 
///             #[proc_macro_attribute]
///             pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
///                 println!("attr: \"{}\"", attr.to_string());
///                 println!("item: \"{}\"", item.to_string());
///                 item
///             }
/// 
/// 
/// 最后在项目中使用:
/// 
/// 
/// 
/// 
/// 
/// 
///             // 使用 hello_procedural_macros 中定义的 【类属性】 过程宏
///             extern crate hello_procedural_macros;
///             use hello_procedural_macros::show_streams;
///             
///             // 示例: 基础函数
///             #[show_streams]
///             fn invoke1() {}
///             
///             // 示例: 带输入参数的属性
///             #[show_streams(bar)]
///             fn invoke2() {}
///             
///             
///             // 示例: 输入参数中有多个 token 的
///             #[show_streams(multiple => tokens)]
///             fn invoke3() {}
///             
///             
///             // 示例:
///             #[show_streams { delimiters }]
///             fn invoke4() {}
///             
///             
///             fn main() {
///             
///                 //  attr: ""
///                 //  item: "fn invoke1() {}"
///                 invoke1();
///             
///                 //  attr: "bar"
///                 //  item: "fn invoke2() {}"
///                 invoke2();
///             
///                 //  attr: "multiple => tokens"
///                 //  item: "fn invoke3() {}"
///                 invoke3();
///             
///                 //  attr: "delimiters"
///                 //  item: "fn invoke4() {}"
///                 invoke4();
///             
///             }
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 【3.【派生】 过程宏】
/// 
/// 
/// 
/// 派生过程宏 为派生(derive)属性定义新输入。
/// 
///     这类宏在给定输入结构体(struct)、枚举(enum)或联合体(union) token流的情况下创建新程序项。    (derive过程宏只能用在 struct 和 enum  和  union上)
/// 
///     它们也可以定义派生宏辅助属性。
///     (辅助属性的定义方式是向 proc_macro_derive 属性增加 attributes(helper0, helper1, ..) 参数，该参数可包含用逗号)
/// 
/// 
/// 
/// 自定义派生宏由带有 proc_macro_derive属性和 (TokenStream) -> TokenStream签名的公有可见性函数定义
/// 
/// 
/// 
/// 
/// 
/// 
/// 如, 在 hello_procedural_macros 的 Cargo.toml 中添加:  
/// 
///             [lib]
///             proc-macro = true
/// 
/// 然后在 hello_procedural_macros 中定义:  
/// 
/// 
/// 
/// 
/// 
///             // 定义  【派生】 过程宏
///             use quote::quote;
///             use syn;
///             use syn::DeriveInput;
///             
///             #[proc_macro_derive(HelloMacro)]
///             pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
///                 
///                 // 基于 input 构建 AST 语法树
///                 let ast:DeriveInput = syn::parse(input).unwrap();
///             
///                 // 构建特征实现代码
///                 impl_hello_macro(&ast)
///             }
///             
///             fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
///                 
///                 // 将 struct 名称赋值给 name 变量
///                 let name = &ast.ident;
///                 
///                 // 可以定义我们想要返回的 Rust 代码
///                 let gen = quote! {
///             
///                     // #name 是 quote lib 的语法
///                     //
///                     // 此处为使用 了 #[derive(HelloMacro)] 的 struct 将被实现 trait HelloTrait 的 fn hello_macro 函数
///                     //
///                     // 值得注意的是， trait HelloTrait 必须在当前 派生过程宏 HelloMacro 使用的 scope 内能找到定义
///                     impl HelloTrait for #name {
///             
///                         fn hello_macro() {
///                             println!("Hello, Macro! My name is {:?}!", stringify!(#name));
///                         }
///                     }
///                 };
///             
///                 // 由于编译器需要的内容和 quote! 直接返回的不一样，因此还需要使用 .into 方法其转换为 TokenStream
///                 gen.into()
///             
///                 // 上述可以这样：
///                 //
///                 //      let ret = gen.into();  // 得到类型为 TokenStream
///                 //      
///                 //      ret
///                 // 查看返回信息
///             }
/// 
/// 
/// 
/// 
/// 
/// 
/// 最后在项目中使用:
/// 
/// 
/// 
/// 
/// 
/// 
/// 
///             // 使用 hello_procedural_macros 中定义的 【派生】 过程宏
///             extern crate hello_procedural_macros;
///             use hello_procedural_macros::HelloMacro;
///             
///             #[derive(HelloMacro)]
///             struct SomeData (u32, String);
///             
///             
///             #[derive(HelloMacro)]
///             struct SomeData2 {
///                 name: String
///             }
///             
///             // 因为 派生过程宏 HelloMacro 中使用了 trait HelloTrait
///             // 所以 派生过程宏 HelloMacro 被使用的 scope 内必须能找到 trait HelloTrait 的定义
///             trait HelloTrait  {
///                 fn hello_macro();
///             }
///             
///             fn main() {
///                
///                 SomeData::hello_macro();        // Hello, Macro! My name is SomeData!
///                 SomeData2::hello_macro();       // Hello, Macro! My name is SomeData2!
///             
///             }
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// --------------------------------------- 高级的 【derive 过程宏】 定义 -------------------------------------------------
/// 
/// 
/// 
/// 如, 在 hello_procedural_macros 的 Cargo.toml 中添加:  
/// 
///             [lib]
///             proc-macro = true
/// 
/// 然后在 hello_procedural_macros 中定义:  
/// 
/// 
/// 
/// 
/// 
///             // 定义  【派生】 过程宏
///             use quote::quote;
///             use syn::{self, Data};
///             use syn::DeriveInput;
/// 
/// 
/// 
///             #[proc_macro_derive(MyDefaultMacro)]
///             pub fn my_default(input: TokenStream) -> TokenStream {
///             
///                 let ast: DeriveInput = syn::parse(input).unwrap();
///             
///                 // 将 struct 名称赋值给 id 变量
///                 let id = ast.ident;
///                 //id = Ident { ident: "SomeData", span: #0 bytes(248..256) }
///                 //id = Ident { ident: "User", span: #0 bytes(305..309) }
///                 // println!("id = {:?}", id);
///             
///                 // let name = &ast.ident;
///                 // //name = Ident { ident: "SomeData", span: #0 bytes(248..256) }
///                 // //name = Ident { ident: "User", span: #0 bytes(305..309) }
///                 // println!("name = {:?}", name);
///             
///             
///                 // 从解析出来的 ast 上读取出 struct 的信息
///                 let Data::Struct(s) = ast.data else {
///                     panic!("MyDefault derive macro must use in struct");
///                 };
///             
///             
///                 // 声明一个新的 ast，用于动态构建字段赋值的 token
///                 let mut field_ast = quote!();
///             
///                 // 这里就是要动态添加 token 的地方了，需要动态完成 Self 的字段赋值
///                 //
///                 // 逐个遍历 struct 上的 field
///                 for (idx,f) in s.fields.iter().enumerate() {
///             
///             
///                    
///             
///                     let (field_name, field_ty) = (&f.ident, &f.ty);
///                     
///                     // println!("field_name = {:?}", field_name);
///             
///                     if field_name.is_none(){
///                         
///                          // 没有 ident 表示是匿名字段，对于匿名字段，都需要添加 `#_field_idx: #field_type::default(),` 这样的代码
///                          //
///                          // 像 struct SomeData (u32,String); 这样的 field 信息就是 None
///                         let _field_idx  = syn::Index::from(idx);
///                         field_ast.extend(quote! {
///                             #_field_idx: #field_ty::default(),
///                         });
///                     }else{
///                         
///                         // 对于命名字段，都需要添加 `#field_name: #field_type::default(),` 这样的代码
///                         //
///                         // 像
///                         //
///                         //  struct User {
///                         //      name: String,
///                         //      data: SomeData,
///                         //  }
///                         field_ast.extend(quote! {
///                             #field_name: #field_ty::default(),
///                         });
///                     }
///                 }
///                 
///                 quote! {
///                     impl MyDefaultTrait for # id {
///                         fn default() -> Self {
///                             
///                             // 返回自身
///                             Self {
///                                 #field_ast
///                             }
///                         }
///                     }
///                 }.into()
///                 
///             }
/// 
/// 
/// 
/// 
/// 最后在项目中使用:
/// 
/// 
/// 
/// 
/// 
/// 
/// 
///             // 使用 hello_procedural_macros 中定义的 【派生】 过程宏
///             extern crate hello_procedural_macros;
///             use hello_procedural_macros::MyDefaultMacro;
///             
///             #[derive(MyDefaultMacro)]
///             struct SomeData (u32,String);
///             
///             #[derive(MyDefaultMacro)]
///             struct User {
///                 name: String,
///                 data: SomeData,
///             }
///             
///             
///             // 因为 派生过程宏 HelloMacro 中使用了 trait HelloTrait
///             // 所以 派生过程宏 HelloMacro 被使用的 scope 内必须能找到 trait HelloTrait 的定义
///             trait MyDefaultTrait  {
///                 fn default() -> Self;
///             }
///             
///             fn main() {
///                
///                 SomeData::default();       
///                 User::default();       
///             
///                 // 在项目根目录使用: cargo expand --bin hello_cargo_rusts 展开 MyDefaultMacro 宏，得到：
///                 //
///                 //
///                 //      #![feature(prelude_import)]
///                 //      #[prelude_import]
///                 //      use std::prelude::rust_2021::*;
///                 //      #[macro_use]
///                 //      extern crate std;
///                 //      extern crate hello_procedural_macros;
///                 //      use hello_procedural_macros::MyDefaultMacro;
///                 //      struct SomeData(u32, String);
///                 //      impl MyDefaultTrait for SomeData {
///                 //          fn default() -> Self {
///                 //              Self {
///                 //                  0: u32::default(),
///                 //                  1: String::default(),
///                 //              }
///                 //          }
///                 //      }
///                 //      struct User {
///                 //          name: String,
///                 //          data: SomeData,
///                 //      }
///                 //      impl MyDefaultTrait for User {
///                 //          fn default() -> Self {
///                 //              Self {
///                 //                  name: String::default(),
///                 //                  data: SomeData::default(),
///                 //              }
///                 //          }
///                 //      }
///                 //      trait MyDefaultTrait {
///                 //          fn default() -> Self;
///                 //      }
///                 //      fn main() {
///                 //          SomeData::default();
///                 //          User::default();
///                 //      }
///             
///             }
/// 
/// 
/// 