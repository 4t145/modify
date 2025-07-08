use darling::{FromDeriveInput, FromField, FromVariant};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, parse_macro_input};

#[derive(FromDeriveInput)]
#[darling(
    attributes(modify),
    forward_attrs(allow, doc, cfg),
    supports(struct_named)
)]
pub struct DeriveModificationAttribute {
    ident: syn::Ident,
    target: Box<syn::Type>,
    generics: syn::Generics,
    data: darling::ast::Data<DeriveModificationVariantAttribute, DeriveModificationFieldAttribute>,
}

#[derive(FromField)]
#[darling(attributes(modify), forward_attrs(allow, doc, cfg))]
pub struct DeriveModificationFieldAttribute {
    ident: Option<syn::Ident>,
    by: Option<syn::Expr>,
    on: Option<syn::Ident>,
}

#[derive(FromVariant)]
#[darling(attributes(modify), forward_attrs(allow, doc, cfg))]
pub struct DeriveModificationVariantAttribute {
    ident: syn::Ident,
    fields: darling::ast::Fields<DeriveModificationFieldAttribute>,
}

pub fn generate_modify_exprs(
    fields: darling::ast::Fields<DeriveModificationFieldAttribute>,
) -> TokenStream {
    let mut exprs = Vec::new();
    for (index, field) in fields.fields.into_iter().enumerate() {
        let DeriveModificationFieldAttribute { by, ident, on } = field;
        let ident = ident.map(|i| quote! { #i }).unwrap_or_else(|| {
            let index = syn::Index {
                index: index as u32,
                span: Span::call_site(),
            };
            quote! { #index }
        });
        let by = by.map(|by| quote! { (#by) }).unwrap_or(quote! {()});
        let on = on.map(|on| quote! { &mut target.#on }).unwrap_or(quote! {target});
        exprs.push(quote! {
            #by.finally(self.#ident).modify(#on);
        });
    }
    quote! {
        #(
            #exprs
        )*
    }
}

pub fn parse(derive_input: syn::DeriveInput) -> syn::Result<TokenStream> {
    use syn::ItemImpl;
    let DeriveModificationAttribute {
        ident,
        target,
        generics,
        data,
    } = DeriveModificationAttribute::from_derive_input(&derive_input)?;
    match data {
        darling::ast::Data::Enum(variants) => {
            // for variant in variants {
            //     let variant_ident = variant.ident;
            //     match variant.fields.style {
            //         darling::ast::Style::Tuple => todo!(),
            //         darling::ast::Style::Struct => {
            //             let exprs = generate_modify_exprs(variant.fields);
            //             return Ok(quote! {
            //                 impl #generics modify::Modification<#target> for #ident #generics {
            //                     fn modify(self, target: &mut #target) {
            //                         match self {
            //                             #ident::#variant_ident { #(#exprs)* } => {}
            //                         }
            //                     }
            //                 }
            //             });
            //         },
            //         darling::ast::Style::Unit => {
            //             quote! { #ident::#variant_ident => {} }
            //         }
            //     }
            // }
            // don't support enums for now
            return Err(syn::Error::new_spanned(
                ident,
                "Modification derive does not support enums",
            ));
        }
        darling::ast::Data::Struct(fields) => {
            let mut exprs = generate_modify_exprs(fields);
            Ok(quote! {
                impl #generics modify::Modification<#target> for #ident #generics {
                    fn modify(self, target: &mut #target) {
                        use modify::*;
                        #exprs
                    }
                }
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_struct_style() {
        let my_derive: syn::DeriveInput = syn::parse_quote! {
            #[derive(Modification)]
            #[modify(target = "MyData")]
            pub struct MyUpdate {
                #[modify(by = Extend, on = new_items)]
                new_items: Vec<String>,
                #[modify(by = Set, on = update_time)]
                update_time: SystemTime,
            }
        };
        let output = parse(my_derive).unwrap();
        println!("{output}")
    }

    // #[test]
    // pub fn test_tuple_style() {
    //     let my_derive: syn::DeriveInput = syn::parse_quote! {
    //         #[derive(Modification)]
    //         #[modify(target = "MySturct<T>")]
    //         struct MyUpdate<T> (
    //             #[modify(by = modify::Set)]
    //             String,
    //             #[modify(by = modify::Set)]
    //             i32,
    //             #[modify(by = modify::Set)]
    //             T,
    //             #[modify(by = modify::Extend, field = items)]
    //             i32
    //         );
    //     };
    //     let output = parse(my_derive).unwrap();
    //     println!("{output}")
    // }

    // #[test]
    // pub fn test_unit_style() {
    //     let my_derive: syn::DeriveInput = syn::parse_quote! {
    //         #[derive(Modification)]
    //         #[modify(target = "MySturct<T>")]
    //         struct MyUpdate;
    //     };
    //     let output = parse(my_derive).unwrap();
    //     println!("{output}")
    // }
}
