// Docs: https://doc.rust-lang.org/stable/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as as synstax
    // that we can manipulate
    // unwrap to panic if the call to the syn::parse function fails,
    // becase proc_macro_derive functions must return TokenStream rather than Result
    // to conform to the procedural macro API. In production code, we should provide
    // more specific error messages about what went wrong by using panic! or expect
    // TODO: But doesn't the parsing happen at compile time, why panic at runtime?
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

// hello_macro_derive is responsible for parsing the TokenStream
// impl_hello_macro is responsible for transforming the syntax tree
// The code in the outer function (hello_macro_derive in this case) will be the same for
// almost every procedural macro crate we see or create
// The code in the inner function (impl_hello_macro in this case) will be different depending on
// macros purpose

// proc_macro crate comes with Rust and is the compiler's API that allows us to read and
// manipulate Rust code from our code
// syn crate parses Rust code from a string into a data structure that we can perform operations on
// quote crate turns syn data structures back into Rust code.

// These crates make it much simpler to parse any sort of Rust code we might want to handle

// hello_macro_derive function will be called when a use of our library specifies #[derive(HelloMacro)]

// Result of parsing the struct Pancakes in hello_macro_derive: (Relevant parts; DeriveInput struct)
// DeriveInput {
//     // --snip--
//
//     ident: Ident {
//         ident: "Pancakes",
//         span: #0 bytes(95..103)
//     },
//     data: Struct(
//         DataStruct {
//             struct_token: Struct,
//             fields: Unit,
//             semi_token: Some(
//                 Semi
//             )
//         }
//     )
// }

// Fiels of this struct show that the Rust code we've parsed is a unit struct with the ident (identifier, the name)
// of Pancakes. More information about the other fiels can be found here: https://docs.rs/syn/1.0/syn/struct.DeriveInput.html

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

// Ident struct instance containing the name (indentifier) of annotated tupe using ast.ident,
// The ident we will get will have the ident field with a value of "Pancakes". Thus the name variable
// will contain an Ident struct instance that, when printed, will be the string "Pancakes", the
// name of the struct

// quote! macro lets us define the Rust code that we want to return. The compiler expects something
// different to the direct result of the quote! macro execution, so we need to convert it to a TokenStream type

// quote! macro also provides some templating mechanics: we enter #name, and quote! will replace it with the
// value in the variable name, even repetion similar to regular macros is possible
// More info: https://docs.rs/quote

// We want our procedural macro to generate an implementation of our HelloMacro trait for
// the type the use annotated, which we can get by using #name. The trait implementation has one function,
// hello_macro, whose body contains the fucntionality we want to provide:
// Printing "Hello, Macro! My name is " and then the name of the annotated type

// stringify! macro is built into Rust and takes a Rust expression such as 1 + 2, and at compile time
// turns the expression into a string literal, such as as "1 + 2", which is different to format! or println!
// which evaluate the expression and then return the result into a String
// stringify! also save an allocation by covnerting #name to a string literal at compile time

// See pancakes crate for use of our HelloMacro

// Attribute-like macros
// Docs: https://doc.rust-lang.org/stable/book/ch19-06-macros.html#attribute-like-macros

// Similar to custom dervie macros, but instead of generating code for the derive attribute,
// they allo us to create new attributes. They are also more flexible:
// derive only works for structs and enums; attributes can be applied to other items as well,
// such as functions
// Example with an attribute named route that annotates functions when using a web application framework

// #[route(GET, '/')]
// fn index() {}}

// The signature of the macro definition would look like this

// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

// The first parameter is the `GET, "/"` part. The second is the body of the item
// the attribute is attached to: in this case, fn index() {}

// Function-like macros
// Docs: https://doc.rust-lang.org/stable/book/ch19-06-macros.html#function-like-macros

// Function-like macros define macros that look like function calls.
// Similarly to macro_rules! macros, they're more flexible than functions; for example,
// they can take an unknown number of arguments.
// However, macro_rules! macros! can be defined only using the mach-like syntax we
// discussed in https://doc.rust-lang.org/stable/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming
// They take a TokenStream parameter and their definition manipulates that TokenStream
// using Rust code as the other two types of procedural macros do.
// An example sql! macro might be called like so:

// let sql - sql!(SELECT * FROM posts WHERE id=1);

// This macro would parse the SQL statement inside it and check that it's syntactically correct,
// which is much more complex processing than a macro_rules! macro can do
// The sql! macro would be defined like this:

// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {}

// Syntax similar to custom dervie macro's signature:
// We receive the tokens that are inside the parantheses and return the code we wanted to generate
