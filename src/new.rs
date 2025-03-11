use std::str::FromStr;
use proc_macro2::{Span, TokenStream, Ident};
use quote::{quote, quote_spanned};
use syn::{Attribute, Data, Expr, Fields, FnArg, GenericParam, Lifetime, LifetimeParam, Lit, Meta, Pat, PatIdent, PatType, Type};

fn find_attribute<'a>(attributes: &'a Vec<Attribute>, attribute_name: &str, parent_path: &str) -> Option<&'a Attribute> {
  attributes.iter()
    .filter(|attr| attr.path().segments.len() != 0
      && attr.path().segments[attr.path().segments.len() - 1].ident == attribute_name
      && (attr.path().segments.len() <= 1 || attr.path().segments[attr.path().segments.len() - 2].ident == parent_path))
    .nth(0)
}

fn create_ident(ident: String) -> Ident {
  Ident::new(ident.as_str(), Span::call_site())
}

fn create_lifetime_param(lifetime: String) -> GenericParam {
  GenericParam::Lifetime(LifetimeParam::new(Lifetime::new(
      format!("'{}", lifetime).as_str(),
      Span::call_site())))
}

fn create_fn_arg(arg_name: Ident, arg_type: Type) -> FnArg {
  FnArg::Typed(PatType {
    attrs: vec![],
    pat: Box::new(Pat::Ident(PatIdent {
      attrs: vec![],
      by_ref: None,
      mutability: None,
      ident: arg_name,
      subpat: None,
    })),
    colon_token: Default::default(),
    ty: Box::new(arg_type),
  })
}

pub fn new_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let struct_data = match &ast.data {
    Data::Struct(struct_data) => struct_data,
    Data::Enum(_) => panic!("'New' macro does not support enums :("),
    Data::Union(_) => panic!("'New' macro does not support unions :("),
  };

  let mut generic_lifetimes = Vec::new();

  for param in &ast.generics.params {
    match param {
      GenericParam::Lifetime(lifetime_param) =>
        generic_lifetimes.push(lifetime_param.lifetime.ident.clone()),
      GenericParam::Type(_) => todo!(),
      GenericParam::Const(_) => todo!(),
    }
  }

  let generic_lifetime_params = generic_lifetimes.iter()
    .map(|x| create_lifetime_param(x.to_string())).collect::<Vec<GenericParam>>();

  //panic!("{:#?}", generic_lifetime_params);

  let mut typed_args = Vec::new();

  match &struct_data.fields {
    Fields::Named(named_fields) => {
      for field in &named_fields.named {
        let default_value_attribute = find_attribute(&field.attrs, "default", "new-macro");
        let optional_default_value = if let Some(attr) = default_value_attribute {
          match &attr.meta {
            Meta::Path(_) => None,
            Meta::List(list) => Some(list.tokens.clone()),
            Meta::NameValue(name_value) => match name_value.value.clone() {
              Expr::Lit(lit) => match lit.lit {
                Lit::Str(str) => match TokenStream::from_str(str.value().as_str()) {
                  Ok(stream) => Some(stream),
                  Err(err) => {
                    let err_str = err.to_string();
                    Some(quote_spanned! {str.span()=>
                      compile_error!("{}", #err_str)
                    })
                  },
                },
                _ => None,  // TODO report error properly
              }
              _ => None,  // TODO report error properly
            },
          }
        } else {
          None
        };

        match &field.ident {
          Some(field_name) => {
            typed_args.push((field_name.to_string(), field.ty.clone(), optional_default_value))
          }
          None => {}
        }
      }
    }
    _ => {}
  }

  let params = typed_args.iter()
    .filter(|(_, _, default)| default.is_none())
    .map(|(name, arg_type, _)| create_fn_arg(create_ident(name.clone()), arg_type.clone()))
    .collect::<Vec<FnArg>>();

  let body = typed_args.iter()
    .map(|(name, _, default)| {
      let name_id = create_ident(name.clone());
      if let Some(default_value) = default {
        quote! { #name_id: #default_value }
      } else {
        quote! { #name_id }
      }
    })
    .collect::<Vec<TokenStream>>();

  if generic_lifetime_params.len() == 0 {
    quote! {
      impl #name {
        pub fn new( #(#params),* ) -> Self {
          Self { #(#body),* }
        }
      }
    }
  } else {
    quote! {
      impl <#(#generic_lifetime_params),*> #name<#(#generic_lifetime_params),*> {
        pub fn new( #(#params),* ) -> Self {
          Self { #(#body),* }
        }
      }
    }
  }
}
