#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro_attribute]
pub fn katexit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    katexit2(item.into()).into()
}

// This is a copy of https://katex.org/docs/autorender.html (accessed on 2021/8/28)
const KATEX_HEADER: &str = r#"
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.13.13/dist/katex.min.css" integrity="sha384-RZU/ijkSsFbcmivfdRBQDtwuwVqK7GMOw6IMvKyeWL2K5UAlyp6WonmB8m7Jd0Hn" crossorigin="anonymous">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.13.13/dist/katex.min.js" integrity="sha384-pK1WpvzWVBQiP0/GjnvRxV4mOb0oxFuyRxJlk6vVw146n3egcN5C925NCP7a7BY8" crossorigin="anonymous"></script>
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.13.13/dist/contrib/auto-render.min.js" integrity="sha384-vZTG03m+2yp6N6BNi5iM4rW4oIwk5DfcNdFfxkk9ZWpDriOkXX8voJBFrAO7MpVl" crossorigin="anonymous"></script>
<script>
    document.addEventListener("DOMContentLoaded", function() {
        renderMathInElement(document.body, {
          // customised options
          // • auto-render specific keys, e.g.:
          delimiters: [
              {left: '$$', right: '$$', display: true},
              {left: '$', right: '$', display: false},
              {left: '\\(', right: '\\)', display: false},
              {left: '\\[', right: '\\]', display: true}
          ],
          // • rendering keys, e.g.:
          throwOnError : false
        });
    });
</script>
"#;

fn katexit2(item: TokenStream2) -> TokenStream2 {
    if let Ok(mut item) = syn::parse2(item.clone()) {
        match item {
            syn::Item::Const(syn::ItemConst { ref mut attrs, .. })
            | syn::Item::Enum(syn::ItemEnum { ref mut attrs, .. })
            | syn::Item::ExternCrate(syn::ItemExternCrate { ref mut attrs, .. })
            | syn::Item::Fn(syn::ItemFn { ref mut attrs, .. })
            | syn::Item::ForeignMod(syn::ItemForeignMod { ref mut attrs, .. })
            | syn::Item::Impl(syn::ItemImpl { ref mut attrs, .. })
            | syn::Item::Macro(syn::ItemMacro { ref mut attrs, .. })
            | syn::Item::Mod(syn::ItemMod { ref mut attrs, .. })
            | syn::Item::Static(syn::ItemStatic { ref mut attrs, .. })
            | syn::Item::Struct(syn::ItemStruct { ref mut attrs, .. })
            | syn::Item::Trait(syn::ItemTrait { ref mut attrs, .. })
            | syn::Item::TraitAlias(syn::ItemTraitAlias { ref mut attrs, .. })
            | syn::Item::Type(syn::ItemType { ref mut attrs, .. })
            | syn::Item::Union(syn::ItemUnion { ref mut attrs, .. })
            | syn::Item::Use(syn::ItemUse { ref mut attrs, .. }) => {
                attrs.push(syn::parse_quote! { #[doc = #KATEX_HEADER] });
            }
            _ => {}
        }
        return quote! { #item };
    }

    if let Ok(mut file) = syn::parse2::<syn::File>(item) {
        file.attrs
            .push(syn::parse_quote! { #![doc = #KATEX_HEADER] });
        return quote! { #file };
    }

    todo!()
}
