// Book: https://veykril.github.io/tlborm/

// Declarative Macros with macro_rules! for General Metaprogramming

// Similar to a match expression
// Compare a value to patterns that are associated with with particular code,
// the value is  the literal Rust source code passed to the macro; and the code
// associated with each pattern, when matched, replaces the code passed to the macro
// This all happens during compilation

// This isn't possible with functions, because we would need to know the number or type
// of parameters up front

// Unoptimized vec! implementation
// Note that we do not add the exclamation mark
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// #[macro_export] indicates that this macro should be made available whenever
// the crate in which it is defined is brought into scope

// {} -> Body of macro definition
// One arm with the pattern ( $( $x:expr ),* ) similar to match expression,
// followed by => and the block of code associated with this pattern
// If it matches, the associated block of code will be emitted
// In this case we only have one valid way to match; any other pattern will result in
// an error

// First, a set of parentheses encompasses the whole pattern.
// A dollar sign ($) is next, followed by a set of parentheses
// that captures values that match the pattern within the parentheses
// for use in the replacement code. Within $() is $x:expr,
// which matches any Rust expression and gives the expression the name $x.

// ( $( $name_of_expression:expression ),match_multiple_occurences? )

// vec![1, 2, 3] -> the $x pattern matches three times with the three expressions 1, 2 and 3
// Associated arm: temp_vec.push() within $()* is generated for each part that matches
// $() in the pattern zero or more times. The $x is replaced with each expression matched.
// Result:
// {
//     let mut temp_vec = Vec::new();
//     temp_vec.push(1);
//     temp_vec.push(2);
//     temp_vec.push(3);
//     temp_vec
// }

// Procedural Macros for Generating Code from Attributes

// Accept some code as an input, operate on that code, and produce some code as an output rather
// than matching against paterns and replacing the code with outher code a s declarative macros do
// Three kinds: custom deerive, attribute-like and function-like (work similarly)
// Definitions must reside in their own crate with a special crate type (for complex technical reasons)

// use proc_macro;
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {}

// TokenStream defined in proc_macro crate which is included with Rust -> represents a sequence of tokens
// The source code the macro is operating on makes up the input TokenStream, and the code
// the macro produces is the ouput TokenStream. The function also has an attribute attached
// to it that specifies which kind of procedural macro we're creating.

// How to Write a Custom derive Macro

pub trait HelloMacro {
    fn hello_macro();
}

// Defining a procedural macro. At the time of this writing, procedural macros need to be in their own crate.
// Eventually this restriction might be lifted.
// Convention for structuring creates and macro crates is as follows: for a crate named foo,
// a custom derive procedural macro crate is called foo_derive.
// see macro_derive crate

// Both crates are highly related, but have to be puhlished separately.
// We could add macros_derive as a dependency and re-export the procedural macro code.
// But the way we've structured the project makes it possible to for programmers to use macros crate
// even if they don't want the derive functionality
