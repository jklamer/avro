
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

    /// Takes in a struct that implements the write combination of traits and runs the Struct through a Serde Cycle and asserts the result is the same 
    fn freeze_dry<'a, T>(obj: T)  where T : std::fmt::Debug + Serialize + Deserialize<'a> + AvroSchema + Clone + PartialEq  {
        let schema = T::get_schema();
        let mut writer = Writer::new(&schema, Vec::new());
        writer.append_ser(obj.clone()).unwrap();
        let encoded = writer.into_inner().unwrap();
        let reader = Reader::with_schema(&schema, &encoded[..]).unwrap();
        for value in reader {
            let value = value.unwrap();
            assert_eq!(obj, from_value::<T>(&value).unwrap());
        }

    }
    
    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct Test1 {
        a: i32,
        b: String
    }
    
    #[test]
    fn test_smoke_test() {
        // Uses derived schema for the data class 
        let schema = Test1::get_schema();
        let mut writer = Writer::new(&schema, Vec::new());
        let test = Test1 {
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
            assert_eq!(test, from_value::<Test1>(&value.unwrap()).unwrap());
        }
    }

    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct Test2 {
        //Basics test
        a: bool,
        b: i8,
        c: i16,
        d: i32,
        e: u8,
        f: u16,
        g: i64,
        h: f32,
        i: f64,
        j: char,
        k: String
    }




    
}