use proc_macro::TokenStream;
use quote::quote;
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

    let expanded = quote! {
        #[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
        #vis struct #name;

        impl pasaka::PassageImpl for #name {
            type State = #state_ty;

            fn run(&self, #engine_arg, #state_arg) #output {
                #block
            }

            fn box_clone(&self) -> Box<dyn pasaka::PassageImpl<State = Self::State>> {
                Box::new(self.clone())
            }

            fn name(&self) -> &'static str {
                stringify! (#name)
            }
        }
    };
    expanded.into()
}
