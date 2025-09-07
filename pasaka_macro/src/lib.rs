use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, PatType, Type, parse_macro_input};

#[proc_macro_attribute]
pub fn passage(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let vis = &input.vis;
    let block = &input.block;

    let inputs = &input.sig.inputs;
    let output = &input.sig.output;

    let mut inputs_iter = inputs.iter();
    let engine_arg = inputs_iter.next().expect("expected &mut Engine arg");
    let state_arg = inputs_iter.next().expect("expected state arg");
    assert!(inputs_iter.next().is_none(), "expected 2 arguments");

    let state_ty: &Type = match state_arg {
        syn::FnArg::Typed(PatType { ty, .. }) => ty,
        _ => panic!("expected typed state arguments"),
    };

    let register_fn = format_ident!("__register_passage_{}", name);

    let expanded = quote! {
        #[derive(::std::fmt::Debug, ::std::marker::Copy, ::std::clone::Clone, ::pasaka::serde::Serialize, ::pasaka::serde::Deserialize)]
        #vis struct #name;

        impl ::pasaka::PassageImpl for self::#name {
            type State = #state_ty;

            fn run(&self, #engine_arg, #state_arg) #output {
                #block
            }

            fn box_clone(&self) -> ::std::boxed::Box<dyn ::pasaka::PassageImpl<State = Self::State>> {
                ::std::boxed::Box::new(::std::clone::Clone::clone(self))
            }

            fn name(&self) -> &'static str {
                stringify! (#name)
            }
        }

        #[allow(non_snake_case)]
        #[::pasaka::ctor::ctor(crate_path=::pasaka::ctor)]
        fn #register_fn() {
            ::pasaka::register_passage(stringify! (#name), #name);
        }
    };
    expanded.into()
}
