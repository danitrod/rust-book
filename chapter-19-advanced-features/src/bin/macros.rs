// Macros are implemented with macro_rules!
// Let's implement a naive vec! macro
// #[macro_export] is used to export a macro from a lib
macro_rules! naive_vec {
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

fn main() {
    let v = naive_vec!("hello", "world");

    println!("v: {:?}", v);

    // Implementing a custom derive macro.
    // As of now, procedural macros need a special crate to be created.
    // We will create a lib that is defined to use procedural macros,
    // and another for defining a trait.
    // Find the latter in ../../hello_macro
    use hello_macro::HelloMacro;

    struct Pancakes;

    impl HelloMacro for Pancakes {
        fn hello_macro() {
            println!("Hello from Pancakes");
        }
    }

    Pancakes::hello_macro();

    // To implement the derive macro for this trait, we use the proc crate
    // The convention is to name it with the same name of crate, and suffix _derive
    // Find it at ../../hello_macro/hello_macro_derive
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct DerivedPancakes;

    DerivedPancakes::hello_macro();

    // Other procedural macros
    // Other attribute-like macros similar to derive can be created
    // derive can only be used for structs and enums,
    // custom attributes can be used for functions and other items as well.
    // For example:
    // #[route(GET, "/")]
    // fn get() {}
    //
    // This could be created with:
    // #[proc_macro_attribute]
    // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Here, attr are GET and "/" for the route attribute, and item is the actual function

    // Function-like macros are similar to macros created with macro_rules!
    // but are more flexible, as they don't have to use a match-like syntax.
    // They also take a TokenStream and return a TokenStream.
    // For example, one could implement an SQL macro:
    // let sql = sql!(SELECT * FROM posts WHERE id=1);
    // Validating the SQL syntax is something more complicated, that macro_rules! couldn't do
    // by itself.
    // A function-like macro could be defined like this:
    // #[proc_macro]
    // pub fn sql(input: TokenStream) -> TokenStream {
}
