#![feature(proc_macro_diagnostic)]
extern crate proc_macro;

use proc_macro2::{TokenStream, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta, NestedMeta, Lit, Path, Attribute, Data, LitBool, Expr, ExprLit, Ident};
use syn::spanned::Spanned;
use std::collections::HashMap;

//mod expand;
//mod input;
//mod parse;
//mod util;

fn fail(span: Span, msg: String) -> proc_macro::TokenStream {
	span.unwrap().error(msg).emit();
	return proc_macro::TokenStream::new();
}

//fn warn(span: Span, msg: &str) {
//	span.unwrap().warning(msg).emit();
//}

#[proc_macro_derive(Values, attributes(values))]
pub fn derive_values(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let expanded = match expand(input) {
		Ok(expanded) => expanded,
		Err(err) => return fail(err.0, err.1),
	};

	proc_macro::TokenStream::from(expanded)
}

struct ExpandError(Span, String);
type ExpandResult<T> = Result<T, ExpandError>;

fn err<T>(span: Span, msg: String) -> ExpandResult<T> {
	ExpandResult::Err(ExpandError(span, msg))
}

fn expand(input: DeriveInput) -> ExpandResult<TokenStream> {
	let name = input.ident;

	let fields = parse_fields(&input.attrs)?;
	let (variants, num_vars) = collect_variants(&input.data, &fields)?;

//	dbg!(input.data);

	let mut methods = Vec::new();
	let empty = HashMap::new();

	for (method_name, (method_name_ident, return_type)) in fields {
		let variants = variants.get(&method_name).unwrap_or(&empty);

		let mut variants_out = Vec::new();

		for (variant_name, variant_expr) in variants {
			if return_type.is_ident("String") {
				variants_out.push(quote!(#variant_name => #variant_expr.to_string()));
			} else {
				variants_out.push(quote!(#variant_name => #variant_expr));
			}
		}

		if variants_out.len() < num_vars {
			variants_out.push(quote!(_ => Default::default()));
		}

		methods.push(quote! {
			pub fn #method_name_ident(&self) -> #return_type {
				use #name::*;

				match self {
					#(#variants_out),*
				}
			}
		});
	}

	let expanded = quote! {
		impl #name {
			#(#methods)*
		}
	};

	Ok(expanded)
}

fn collect_variants(data: &Data, fields: &HashMap<String, (Ident, Path)>) -> ExpandResult<(HashMap<String, HashMap<Ident, Expr>>, usize)> {
	let data_enum = match data {
		Data::Enum(e) => e,
		Data::Struct(s) => return err(s.struct_token.span(), format!("Only enums allowed"))?,
		Data::Union(u) => return err(u.union_token.span(), format!("Only enums allowed"))?,
	};

	let mut variants = HashMap::new();

	for variant in data_enum.variants.iter() {
		let attrs = parse_attributes(&variant.attrs)?;

		for (name, (_, lit)) in attrs {
			let parsed_lit = match &lit {
				Lit::Str(string) => {
					if fields.get(&name).unwrap().1.is_ident("String") {
						Expr::Lit(ExprLit {
							attrs: vec![],
							lit,
						})
					} else {
						match string.parse::<Expr>() {
							Ok(lit) => lit,
							Err(error) => return err(lit.span(), format!("Fd {}", error))?,
						}
					}
				},
				Lit::Bool(_) => Expr::Lit(ExprLit {
					attrs: vec![],
					lit,
				}),
				_ => return err(lit.span(), format!("Only strings allowed"))?,
			};

			let field_entry = variants.entry(name).or_insert_with(|| {
				HashMap::new()
			});

			field_entry.insert(variant.ident.clone(), parsed_lit);
		}
	}

	Ok((variants, data_enum.variants.len()))
}

fn parse_fields(attrs: &Vec<Attribute>) -> ExpandResult<HashMap<String, (Ident, Path)>> {
	let attributes = parse_attributes(attrs)?;

	let mut fields = HashMap::new();

	for (ident_str, (ident, literal)) in attributes {
		let value = match &literal {
			Lit::Str(string) => string.parse::<Path>(),
			_ => return err(literal.span(), format!("Value should be a string"))?,
		};

		let value = match value {
			Ok(p) => p,
			_ => return err(literal.span(), format!("Value should be a valid type"))?,
		};

		fields.insert(ident_str, (ident, value));
	}

	Ok(fields)
}

fn parse_attributes(attrs: &Vec<Attribute>) -> ExpandResult<HashMap<String, (Ident, Lit)>> {
	let mut attributes = HashMap::new();

	for attr in attrs.iter() {
		let meta = match attr.parse_meta() {
			Ok(meta) => meta,
			Err(error) => return err(attr.span(), format!("Failed parsing as Meta: {}", error))?,
		};

		let list = match meta {
			Meta::List(list) => list,
			_ => return err(meta.span(), format!("Only list meta is supported"))?,
		};

		if !list.path.is_ident("values") {
			// discard
			continue;
		}

		for entry in list.nested {
			let (path, value) = match entry {
				NestedMeta::Meta(Meta::NameValue(meta)) => (meta.path, meta.lit),
				NestedMeta::Meta(Meta::Path(meta)) => (meta.clone(), Lit::Bool(LitBool { value: true, span: meta.span().clone() })),
				_ => return err(entry.span(), format!("Inner should be name value meta"))?,
			};

			let ident = path.get_ident().expect("Name value key should be ident").clone();
			let ident_str = ident.to_string();

			attributes.insert(ident_str, (ident, value));
		}
	}

	Ok(attributes)
}
