use avro_rs::Schema;
use quote::quote;
use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput};



trait AvroSchema {
    fn get_schema() -> Schema;
}

#[proc_macro_derive(AvroSchema)]
pub fn derive_avro_schema(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);


    let dummy = quote! {let a = 5};
    proc_macro::TokenStream::from(dummy)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
