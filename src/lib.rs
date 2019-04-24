#![recursion_limit = "128"]

extern crate hyper;
extern crate proc_macro;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Attribute, Ident, ItemTrait, Meta, MethodSig, TraitItem, Type};

enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Unknown,
}

impl From<&str> for HttpMethod {
    fn from(s: &str) -> HttpMethod {
        return match s {
            "get" => HttpMethod::Get,
            "post" => HttpMethod::Post,
            "put" => HttpMethod::Put,
            "patch" => HttpMethod::Patch,
            "delete" => HttpMethod::Delete,
            _ => HttpMethod::Unknown,
        };
    }
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct Attrs {
    path: String,
    data: Option<String>,
    encoded: Option<bool>,
}

struct RequestMeta {
    method: HttpMethod,
    attributes: Attrs,
}

#[derive(Debug)]
struct Param {
    ident: Ident,
    ty: Type,
}

#[proc_macro_attribute]
pub fn http_client(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_trait = syn::parse_macro_input!(item as ItemTrait);
    let trait_ident = input_trait.ident;

    let mut methods = vec![];

    for i in input_trait.items {
        match i {
            TraitItem::Method(m) => {
                let args = match get_meta_from_attributes(m.attrs) {
                    Ok(args) => args,
                    Err(e) => return e.write_errors().into(),
                };

                let method_signature = m.sig;
                let params = parse_method_params(&method_signature);

                let segments = args.attributes.path;
                let fmt_args = params
                    .iter()
                    .filter(|p| segments.contains(&format!("{{{}}}", p.ident.to_string())))
                    .map(move |p| {
                        let ident = &p.ident;
                        quote! { #ident = #ident }
                    })
                    .collect::<Vec<proc_macro2::TokenStream>>();

                let build_url = quote! {
                    let path_segments = format!(#segments, #(#fmt_args),*);
                    let mut url = String::new();
                    url.push_str(&self.root);
                    url.push_str(&path_segments);
                };

                let method = match args.method {
                    HttpMethod::Get => quote! {
                        #method_signature {
                            #build_url
                            self.client.get(&url).send().into()
                        }
                    },
                    HttpMethod::Post => match args.attributes.data {
                        Some(data) => {
                            let data_token = if data.starts_with('{') && data.ends_with('}') {
                                let param_name = &data[1..(data.len() - 1)];
                                let matching_param =
                                    params.iter().find(|p| p.ident.to_string() == param_name);
                                match matching_param {
                                    Some(m) => m.ident.clone(),
                                    None => {
                                        return syn::Error::new(
                                            Span::call_site(),
                                            format!("No matching param for {}", data),
                                        )
                                        .to_compile_error()
                                        .into();
                                    }
                                }
                            } else {
                                Ident::new(&data, Span::call_site())
                            };
                            quote! {
                                #method_signature {
                                    #build_url
                                    self.client.post(&url)
                                    .body(#data_token)
                                    .send()
                                    .into()
                                }
                            }
                        }
                        None => {
                            quote! {
                                #method_signature {
                                    #build_url
                                    self.client.post(&url).send().into()
                                }
                            }
                        }
                    },
                    _ => unimplemented!(),
                };
                // println!("{}", method);
                methods.push(method);
                // println!("{:?}", method.to_string());
            }
            _ => {}
        }
    }

    let res = quote! {
        struct #trait_ident {
            root: String,
            client: reqwest::Client
        }
        impl #trait_ident {
            fn new<S: Into<String>>(root: S, client: reqwest::Client) -> Self {
                #trait_ident { root: root.into(), client: client }
            }
            #(#methods)*
        }

    };

    println!("{}", res);
    res.into()
}

fn parse_method_params(method: &MethodSig) -> Vec<Param> {
    let mut params = vec![];

    for input in &method.decl.inputs {
        let param = match input {
            syn::FnArg::Captured(arg) => match &arg.pat {
                syn::Pat::Ident(pat) => Param {
                    ident: pat.ident.clone(),
                    ty: arg.ty.clone(),
                },
                _ => continue,
            },
            _ => continue,
        };
        params.push(param);
    }

    params
}

fn get_meta_from_attributes(attributes: Vec<Attribute>) -> Result<RequestMeta, darling::Error> {
    for attribute in attributes {
        if let Ok(Meta::List(meta)) = attribute.parse_meta() {
            let method = HttpMethod::from(meta.ident.to_string().as_ref());
            match method {
                HttpMethod::Unknown => {
                    return Err(
                        darling::Error::custom("Expected get, post, put, patch or delete")
                            .with_span(&meta),
                    );
                }
                _ => {
                    let parsed_attr = Attrs::from_meta(&meta.into())?;
                    return Ok(RequestMeta {
                        method: method,
                        attributes: parsed_attr,
                    });
                }
            };
        };
    }
    Err(darling::Error::custom("No path attribute found"))
}
