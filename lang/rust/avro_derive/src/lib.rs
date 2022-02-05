use std::collections::HashMap;

use avro_rs::{Schema, schema::{RecordField, RecordFieldOrder, Name}};
use quote::quote;
use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error, Type};

trait AvroSchema {
    fn get_schema() -> &'static Schema;
}

#[proc_macro_derive(AvroSchema)]
pub fn proc_macro_derive_avro_schema(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    derive_avro_schema(&mut input)
    .unwrap_or_else(to_compile_errors)
    .into()
}

fn derive_avro_schema(input: &mut DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let name = input.ident.to_string();
    let mut schema_fields : Vec<RecordField> = vec![];
    match &input.data {
        syn::Data::Struct(s) => {
            match s.fields {
                syn::Fields::Named(ref a) => {
                    for (position, field) in a.named.iter().enumerate() {
                        let name = field.ident.as_ref().unwrap();
                        let ty = &field.ty;
                        schema_fields.push(RecordField{
                            name: name.to_string(),
                            doc: Option::None,
                            default: Option::None,
                            schema: type_to_schema(ty)?,
                            order: RecordFieldOrder::Ignore,
                            position: position,
                        })
                    }
                },
                _ => return Err(vec![]),
            }
        },
        _ => return Err(vec![]),
    };

    let lookup: HashMap<String, usize> =  schema_fields.iter().map(|field | { field.name.to_owned() }).enumerate().map(|(num, name)| { (name, num)}).collect();
    let record_schema = Schema::Record{
        name: Name::new(&name[..]),
        doc: None,
        fields: schema_fields,
        lookup: lookup,
    };
    println!("{}", record_schema.canonical_form());
    let can_form = record_schema.canonical_form();
    let ty = &input.ident;
    let dummy = quote! {
        impl AvroSchema for #ty {
            const schema : Schema = Schema::parse_str(#can_form).unwrap();
            pub fn get_schema() -> &'static Schema {
                &schema
            }
        }
    };
    Ok(dummy)
}

fn type_to_schema(ty: &Type) -> Result<Schema, Vec<Error>> {
    if let Type::Path(p) = ty {
        let type_string = p.path.segments.last().unwrap().ident.to_string();
        // println!("{:?}",type_string);
        // println!("{:?}",ty);

        let schema = match &type_string[..] {
            "i8" | "i16" | "i32" => Schema::Int,
            "i64" => Schema::Long,
            "String" => Schema::String,
            _ => return Err(vec![]),
        };
        Ok(schema)
    } else {
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
    fn it_works() {
        
        let test_struct = quote!{
            struct A {
                a: i32,
                b: String
            }
        };
        
        match syn::parse2::<DeriveInput>(test_struct){
            Ok(mut input) => {
                println!("{}", derive_avro_schema(&mut input).unwrap());
            },
            Err(_) => println!("error!")
        };
    }
}
