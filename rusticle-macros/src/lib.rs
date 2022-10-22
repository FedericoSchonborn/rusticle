#![warn(clippy::pedantic)]

use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_stream(stream: TokenStream, replaced: &mut Vec<TokenTree>) {
    for tree in stream {
        match tree {
            TokenTree::Group(group) => {
                let mut elements = vec![];
                replace_stream(group.stream(), &mut elements);

                replaced.push(TokenTree::Group(Group::new(
                    group.delimiter(),
                    TokenStream::from_iter(elements),
                )));
            }
            TokenTree::Ident(ident) => replaced.push(TokenTree::Ident(Ident::new(
                match &*ident.to_string() {
                    // Keywords
                    "asynchronous" => "async",
                    "constant" => "const",
                    "dynamic" => "dyn",
                    "enumeration" => "enum",
                    "external" => "extern",
                    "function" => "fn",
                    "implement" => "impl",
                    "lettuce" => "let",
                    "module" => "mod",
                    "mutable" => "mut",
                    "public" => "pub",
                    "byreference" => "ref",
                    "itself" => "self",
                    "Itself" => "Self",
                    "structure" => "struct",
                    "superior" => "super",
                    "using" => "use",
                    other => other,
                },
                ident.span(),
            ))),
            TokenTree::Punct(..) | TokenTree::Literal(..) => replaced.push(tree),
        }
    }
}

#[proc_macro]
pub fn rusticle(stream: TokenStream) -> TokenStream {
    let mut replaced = vec![];
    replace_stream(stream, &mut replaced);

    TokenStream::from_iter(replaced)
}
