
// 定义 【类函数】 过程宏
#![crate_type = "proc-macro"]
extern crate proc_macro;
use std::fmt::Pointer;

use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    println!("_item: \"{}\"", _item.to_string());
    "fn answer() -> u32 { 42 }".parse().unwrap()
}


// 定义  【类属性】 过程宏
#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}




// 定义  【派生】 过程宏
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    
    // 基于 input 构建 AST 语法树
    let ast:DeriveInput = syn::parse(input).unwrap();

    // 构建特征实现代码
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    
    // 将 struct 名称赋值给 name 变量
    let name = &ast.ident;
    
    // 可以定义我们想要返回的 Rust 代码
    let gen = quote! {

        // #name 是 quote lib 的语法
        //
        // 此处为使用 了 #[derive(HelloMacro)] 的 struct 将被实现 trait HelloTrait 的 fn hello_macro 函数
        //
        // 值得注意的是， trait HelloTrait 必须在当前 派生过程宏 HelloMacro 使用的 scope 内能找到定义
        impl HelloTrait for #name {

            fn hello_macro() {
                println!("Hello, Macro! My name is {:?}!", stringify!(#name));
            }
        }
    };

    // 由于编译器需要的内容和 quote! 直接返回的不一样，因此还需要使用 .into 方法其转换为 TokenStream
    gen.into()

    // 上述可以这样：
    //
    //      let ret = gen.into();  // 得到类型为 TokenStream
    //      
    //      ret
    // 查看返回信息
}






// use syn::Data;
use syn::{self, Data};  // 等价于 use syn; 和 use syn::Data; 的复合


#[proc_macro_derive(MyDefaultMacro)]
pub fn my_default(input: TokenStream) -> TokenStream {

    let ast: DeriveInput = syn::parse(input).unwrap();

    // 将 struct 名称赋值给 id 变量
    let id = ast.ident;
    //id = Ident { ident: "SomeData", span: #0 bytes(248..256) }
    //id = Ident { ident: "User", span: #0 bytes(305..309) }
    // println!("id = {:?}", id);

    // let name = &ast.ident;
    // //name = Ident { ident: "SomeData", span: #0 bytes(248..256) }
    // //name = Ident { ident: "User", span: #0 bytes(305..309) }
    // println!("name = {:?}", name);


    // 从解析出来的 ast 上读取出 struct 的信息
    let Data::Struct(s) = ast.data else {
        panic!("MyDefault derive macro must use in struct");
    };


    // 声明一个新的 ast，用于动态构建字段赋值的 token
    let mut field_ast = quote!();

    // 这里就是要动态添加 token 的地方了，需要动态完成 Self 的字段赋值
    //
    // 逐个遍历 struct 上的 field
    for (idx,f) in s.fields.iter().enumerate() {


       

        let (field_name, field_ty) = (&f.ident, &f.ty);
        
        // println!("field_name = {:?}", field_name);

        if field_name.is_none(){
            
             // 没有 ident 表示是匿名字段，对于匿名字段，都需要添加 `#_field_idx: #field_type::default(),` 这样的代码
             //
             // 像 struct SomeData (u32,String); 这样的 field 信息就是 None
            let _field_idx  = syn::Index::from(idx);
            field_ast.extend(quote! {
                #_field_idx: #field_ty::default(),
            });
        }else{
            
            // 对于命名字段，都需要添加 `#field_name: #field_type::default(),` 这样的代码
            //
            // 像
            //
            //  struct User {
            //      name: String,
            //      data: SomeData,
            //  }
            field_ast.extend(quote! {
                #field_name: #field_ty::default(),
            });
        }
    }
    
    quote! {
        impl MyDefaultTrait for # id {
            fn default() -> Self {
                
                // 返回自身
                Self {
                    #field_ast
                }
            }
        }
    }.into()
    
}

