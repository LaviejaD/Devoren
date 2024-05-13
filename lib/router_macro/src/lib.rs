use http::Method;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Pat, Type};

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = attr.to_string();
    let args: Vec<&str> = attr.split(",").collect();
    // println!("len {}", args.len());
    if args.len() < 2 {
        panic!("No pass method or uri example: #[get,/index.html]")
    }

    let input = parse_macro_input!(item as ItemFn);
    let name = input.sig.ident;
    let xd = input.sig.inputs.first().unwrap();
    //   dbg!(&xd);
    match xd {
        FnArg::Typed(t) => {
            match *t.pat.clone() {
                Pat::Ident(a) => println!("{}", a.ident.to_string()),
                _ => todo!(),
            }

            match *t.ty.clone() {
                Type::Path(p) => {
                    let ah = p.path.get_ident().unwrap().to_string();
                    println!("{ah}")
                }
                _ => todo!(),
            }
        }
        _ => todo!(),
    };

    let block = input.block;
    let method: Vec<_> = args[0].split_whitespace().collect();
    let method = method.join("");
    let enpoint: Vec<_> = args[1].split_whitespace().collect();
    let endpoint = enpoint.join("");
    // println!("actibutos {} {}", method, uri);
    Method::from(method.clone());

    quote! {
        pub struct #name; impl Route for #name {

        fn call(&self) -> String{
        #block
        }
        fn enpoint(&self) -> (Method,String){

    (Method::from(#method.to_string()),#endpoint.to_string())
        }
            } }
    .into()
}
