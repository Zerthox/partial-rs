use attribute_derive::FromAttr;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, Path};

#[derive(FromAttr)]
#[attribute(ident = partial)]
pub struct Args {
    name: Option<String>,

    #[attribute(optional)]
    derive: Vec<Path>,
}

impl Args {
    pub fn name(&self, parent: &Ident) -> Ident {
        self.name
            .as_ref()
            .map(|name| Ident::new(name, Span::call_site()))
            .unwrap_or_else(|| format_ident!("Partial{parent}"))
    }

    pub fn attributes(&self, parent: &Ident) -> TokenStream {
        let derives = &self.derive;
        let doc = format!("Partial of [`{parent}`].");
        quote! {
            #[doc = #doc]
            #[derive(#(#derives),*)]
        }
    }
}
