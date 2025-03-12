use std::str::FromStr;
use proc_macro2::{Span, TokenStream, TokenTree, Ident, Punct, Spacing};
use quote::{quote, quote_spanned};
use syn::{Attribute, Data, Expr, Fields, FnArg, GenericParam, Lit, Meta, Pat, PatIdent, PatType, Type};

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

fn create_punct(punct: char) -> Punct {
  Punct::new(punct, Spacing::Joint)
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

  let generic_lifetimes_left = &ast.generics.params.iter().map(|x| x.clone()).collect::<Vec<GenericParam>>();
  let generic_lifetimes_right = generic_lifetimes_left.iter()
    .map(|param| match param {
      GenericParam::Lifetime(lifetime_param) => vec![
        TokenTree::Punct(create_punct('\'')),
        TokenTree::Ident(lifetime_param.lifetime.ident.clone()),
      ],
      GenericParam::Type(type_param) => vec![ TokenTree::Ident(type_param.ident.clone()) ],
      GenericParam::Const(const_param) => vec![ TokenTree::Ident(const_param.ident.clone()) ],
    })
    .map(|token_trees| token_trees.into_iter().collect::<TokenStream>())
    .collect::<Vec<TokenStream>>();

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

  if generic_lifetimes_left.len() == 0 {
    quote! {
      impl #name {
        pub fn new( #(#params),* ) -> Self {
          Self { #(#body),* }
        }
      }
    }
  } else {
    quote! {
      impl <#(#generic_lifetimes_left),*> #name<#(#generic_lifetimes_right),*> {
        pub fn new( #(#params),* ) -> Self {
          Self { #(#body),* }
        }
      }
    }
  }
}
