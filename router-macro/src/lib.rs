use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, Parser};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Path, Token};

// let case_check = match Punctuated::<Path, Token![,]>::parse_terminated.parse(args) {

#[proc_macro_attribute]
pub fn component(args: TokenStream, input: TokenStream) -> TokenStream {
    let as_fn = parse_macro_input!(input as syn::ItemFn);

    // parse_macro_input!
    let Arg {
        equals,
        route,
        router,
    } = syn::parse_macro_input!(args as Arg);

    let fn_name = as_fn.sig.ident.clone();
    let inputs = as_fn.sig.inputs.clone();
    quote! {
        impl #router {
            fn #fn_name(
                #inputs
            ) -> () {
                #[linkme::distributed_slice(crate::support::ROUTES)]
                fn info() -> crate::support::RouteInfo {
                    crate::support::RouteInfo {
                       route: #route,
                        id: std::any::TypeId::of::<#router>(),
                    }
                }
            }
        }



        #as_fn
    }
    .to_token_stream()
    .into()
}

struct Arg {
    router: Path,
    equals: Token![=],
    route: syn::LitStr,
}

impl Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            router: input.parse()?,
            equals: input.parse()?,
            route: input.parse()?,
        })
    }
}

// /// General struct for parsing a component body.
// /// However, because it's ambiguous, it does not implement [`ToTokens`](quote::to_tokens::ToTokens).
// ///
// /// Refer to the [module documentation](crate::component_body) for more.
// pub struct ComponentBody {
//     /// The component function definition. You can parse this back into a [`ComponentBody`].
//     /// For example, you might modify it, parse it into a [`ComponentBody`], and deserialize that
//     /// using some deserializer. This is how deserializers use other deserializers.
//     ///
//     /// **`item_fn.sig.inputs` includes the context argument!**
//     /// Keep this in mind when creating deserializers, because you often might want to ignore it.
//     /// That might be annoying, but it would be bad design for this kind of struct to not be parsable from itself.
//     pub item_fn: ItemFn,
//     /// If the function has any arguments other than the context.
//     pub has_extra_args: bool,
// }

// impl ComponentBody {
//     /// Deserializes the body into the [`TOutput`] with the specific [`TArgs`].
//     /// Even if the args are empty, the [`TArg`] type still determines what [`TOutput`] will be generated.
//     pub fn deserialize<TOutput, TArgs>(&self, args: TArgs) -> Result<TOutput>
//     where
//         TOutput: DeserializerOutput,
//         TArgs: DeserializerArgs<TOutput>,
//     {
//         args.to_output(self)
//     }
// }

// impl Parse for ComponentBody {
//     fn parse(input: ParseStream) -> Result<Self> {
//         let item_fn: ItemFn = input.parse()?;

//         let element_type_path = "dioxus_core::Element";

//         if item_fn.sig.output == ReturnType::Default {
//             return Err(Error::new(
//                 item_fn.sig.output.span(),
//                 format!("Must return a <{}>", element_type_path),
//             ));
//         }

//         let has_extra_args = !item_fn.sig.inputs.is_empty();

//         Ok(Self {
//             item_fn,
//             has_extra_args,
//         })
//     }
// }
