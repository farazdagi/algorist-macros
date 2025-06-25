use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{ItemFn, parse_macro_input},
};

/// Marks a function that will run test cases using the
/// `algorist::io::test_cases` function.
///
/// # Example
///
/// ```
/// use algorist_macros::test_cases;
///
/// #[test_cases]
/// pub fn multiple_test_cases() {
///     let (a, b): (i32, i32) = scan.pair();
///     writeln!(w, "Sum: {}", a + b);
/// }
/// ```
/// 
/// Macro will expose `scan` and `w` variables to the function body, where:
/// - `scan` is an instance of `algorist::io::Scanner` for reading input.
/// - `w` is an instance of `std::io::Write` for writing output.
#[proc_macro_attribute]
pub fn test_cases(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    let body = &input.block;

    let code = quote! {
        #vis #sig {
            algorist::io::test_cases(&mut |scan, w| #body);
        }
    };
    code.into()
}

/// Marks a function that will run a single test case using the
/// `algorist::io::test_case` function.
///
/// # Example
///
/// ```
/// use algorist_macros::test_case;
///
/// #[test_case]
/// pub fn single_test_case() {
///     let (a, b): (i32, i32) = scan.pair();
///     writeln!(w, "Sum: {}", a + b);
/// }
/// ```
///
/// Macro will expose `scan` and `w` variables to the function body, where:
/// - `scan` is an instance of `algorist::io::Scanner` for reading input.
/// - `w` is an instance of `std::io::Write` for writing output.
#[proc_macro_attribute]
pub fn test_case(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    let body = &input.block;

    let code = quote! {
        #vis #sig {
            algorist::io::test_case(&mut |scan, w| #body);
        }
    };
    code.into()
}
