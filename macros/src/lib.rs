use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse};

struct MakeRouteArgs {
    name: syn::Ident,
}

impl Parse for MakeRouteArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;

        Ok(Self {
            name,
        })
    }
}

#[proc_macro]
pub fn make_route_svg(input: TokenStream) -> TokenStream {
    let MakeRouteArgs { name } = syn::parse(input).unwrap();

    let file_path = format!("templates/{name}.svg");
    let const_name = format_ident!("svg_data_{name}");
    let fn_name = format_ident!("get_{name}_svg");
    let uri = format!("/<board>/{name}.svg");

    let generated = quote! {
        const #const_name: &'static str = include_str!(#file_path);
        #[get(#uri)]
        pub fn #fn_name(board: u32, state: &State<BoardState>) -> Option<(ContentType, String)> {
            Some((
                ContentType::SVG,
                #const_name
                    .replace("DEADBEEF", &get_board_color_hex(state, board)?)
            ))
        }
    };

    generated.into()
}
