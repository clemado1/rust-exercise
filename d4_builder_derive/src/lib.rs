extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

#[proc_macro_derive(Setter)]
pub fn setter_derive(input: TokenStream) -> TokenStream {
    let mut top = input.into_iter();
    let mut ttype: TokenTree = top.next().unwrap();

    if ttype.to_string() == "pub".to_string() {
        ttype = top.next().unwrap();
    }

    let name = top.next().unwrap();

    format!(
        "impl {} {{
            fn dothing(&self) {{
                println!(\"Doing thing with {0}\");
            }}
    }}", name).parse().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
