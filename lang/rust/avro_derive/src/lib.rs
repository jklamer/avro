use std::collections::HashMap;

use avro_rs::{Schema, schema::{RecordField, RecordFieldOrder, Name, AvroSchema}};
use quote::quote;
use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error, Type, spanned::Spanned};

#[proc_macro_derive(AvroSchema)]
pub fn proc_macro_derive_avro_schema(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    derive_avro_schema(&mut input)
    .unwrap_or_else(to_compile_errors)
    .into()
}

fn derive_avro_schema(input: &mut DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let name = input.ident.to_string();
    let mut record_field_exprs = vec![];
    match &input.data {
        syn::Data::Struct(s) => {
            match s.fields {
                syn::Fields::Named(ref a) => {
                    for (position, field) in a.named.iter().enumerate() {
                        let name = field.ident.as_ref().unwrap().to_string(); // we know everything has a name
                        let schema_expr = type_to_schema_expr(&field.ty)?;
                        let position = position;
                        record_field_exprs.push(quote!{
                            avro_rs::schema::RecordField {
                                    name: #name.to_string(),
                                    doc: Option::None,
                                    default: Option::None,
                                    schema: #schema_expr,
                                    order: avro_rs::schema::RecordFieldOrder::Ignore,
                                    position: #position,
                                }
                        });
                    }
                },
                syn::Fields::Unnamed(_) => return Err(vec![ Error::new(input.ident.span(), "AvroSchema derive does not work for tuple structs")]),
                syn::Fields::Unit => return Err(vec![ Error::new(input.ident.span(), "AvroSchema derive does not work for unit structs")]),  
            }
        },
        _ => return Err(vec![ Error::new(input.ident.span(), "AvroSchema derive only works for structs") ]),
    };

    let schema_def = quote!{
        let schema_fields = vec![#(#record_field_exprs),*];
        avro_rs::schema::record_schema_for_fields(avro_rs::schema::Name::new(#name),None, schema_fields)
    };


    // let lookup: HashMap<String, usize> =  schema_fields.iter().map(|field | {(field.name.to_owned(),  field.position)}).collect();
    // let record_schema = Schema::Record {
    //     name: Name::new(&name[..]),
    //     doc: None,
    //     fields: schema_fields,
    //     lookup: lookup,
    // };
    // println!("{}", record_schema.canonical_form());
    // let can_form = record_schema.canonical_form();
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl(); 
    let dummy = quote! {
        impl #impl_generics AvroSchema for #ty #ty_generics #where_clause {
            const SCHEMA : &'static Schema = &{#schema_def};
        }
    };
    Ok(dummy)
}

fn type_to_schema_expr(ty: &Type) -> Result<TokenStream, Vec<Error>> {
    if let Type::Path(p) = ty {
        let type_string = p.path.segments.last().unwrap().ident.to_string();
        // println!("{:?}",type_string);
        // println!("{:?}",ty);

        let schema = match &type_string[..] {
            "bool" => quote!{Schema::Boolean},
            "i8" | "i16" | "i32" | "u8" | "u16" => quote!{Schema::Int},
            "i64" => quote!{Schema::Long},
            "f32" => quote!{Schema::Float},
            "f64" => quote!{Schema::Double},
            "String" => quote!{Schema::String},
            "char" => return Err(vec![Error::new_spanned(ty, "AvroSchema: Cannot guarentee sucessful deserialization of this type")]),
            "u32" | "u64" => return Err(vec![Error::new_spanned(ty, "Cannot guarentee sucessful serialization of this type due to overflow concerns")]), //Can't guarentee serialization type 
            _ => return Ok(quote!{ #p::get_owned_schema()}),
        };
        Ok(schema)
    }else if let Type::Array(ta) = ty {
        let inner_schema_expr = type_to_schema_expr(&ta.elem)?;
        Ok(quote!{Schema::Array(Box::new(#inner_schema_expr))})
    }
    else {
        Err(vec![])
    }
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn basic_case() {
        
        let test_struct = quote!{
            struct A {
                a: i32,
                b: String
            }
        };
        
        match syn::parse2::<DeriveInput>(test_struct){
            Ok(mut input) => {
                println!("{}", derive_avro_schema(&mut input).unwrap());
                assert!(derive_avro_schema(&mut input).is_ok())
            },
            Err(_) => assert!(false)
        };
    }

    #[test]
    fn tuple_struct_unsupported() {
        let test_tuple_struct = quote!{
            struct B (i32, String);
        };

        match syn::parse2::<DeriveInput>(test_tuple_struct){
            Ok(mut input) => {
                assert!(derive_avro_schema(&mut input).is_err())
            },
            Err(_) => assert!(false)
        };
    }

    #[test]
    fn unit_struct_unsupported() {
        let test_tuple_struct = quote!{
            struct AbsoluteUnit;
        };

        match syn::parse2::<DeriveInput>(test_tuple_struct){
            Ok(mut input) => {
                assert!(derive_avro_schema(&mut input).is_err())
            },
            Err(_) => assert!(false)
        };
    }
}
