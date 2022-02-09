
use avro_derive::*;
use serde::ser::Serialize;
use serde::de::Deserialize;
use avro_rs::schema::AvroSchema;
use avro_rs::Schema;
use avro_rs::Writer;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate avro_derive;

#[cfg(test)]
mod test_derive {
    use avro_rs::{Reader, from_value};

    use super::*;
    
    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct Test {
        a: i32,
        b: String
    }
    
    #[test]
    fn test_feeze_dry() {
        // Uses derived schema for the data class 
        let schema = Test::get_schema();
        let mut writer = Writer::new(&schema, Vec::new());
        let test = Test {
            a: 27,
            b: "foo".to_owned(),
        };
        // successfully writes with the derived schema
        writer.append_ser(test.clone()).unwrap();
        let encoded = writer.into_inner().unwrap();
        // successfully reads with the derived schema
        let reader = Reader::with_schema(&schema, &encoded[..]).unwrap();
        for value in reader {
            //assert we get it back!
            assert_eq!(test, from_value::<Test>(&value.unwrap()).unwrap());
        }
    }
}