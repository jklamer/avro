
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
    use serde::{Deserializer, de::DeserializeOwned};

    use super::*;

    /// Takes in a struct that implements the right combination of traits and runs the Struct through a Serde Cycle and asserts the result is the same 
    fn freeze_dry<T>(obj: T) where T : std::fmt::Debug + Serialize + DeserializeOwned + AvroSchema + Clone + PartialEq  {
        let schema = T::get_schema();
        let mut writer = Writer::new(&schema, Vec::new());
        if let Err(e) = writer.append_ser(obj.clone()) {
            panic!("{}", e.to_string());
        }
        let encoded = writer.into_inner().unwrap();
        let reader = Reader::with_schema(&schema, &encoded[..]).unwrap();
        for res in reader {
            let value = res.unwrap();
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
        let test = Test1 {
            a: 27,
            b: "foo".to_owned(),
        };
        freeze_dry(test);
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
        j: String
    }

    #[test]
    fn test_basic_types() {
        let all_basic = Test2 {
            a: true,
            b: 8,
            c: 16,
            d: 32,
            e: 8,
            f: 16,
            g: 64,
            h: 32.3333,
            i: 64.4444,
            j: "testing string".to_owned(),
        };
        freeze_dry(all_basic);
    }

    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct Test3 {
        a : i32,
        b : Test2
    }

    #[test]
    fn test_inner_struct() {
        let all_basic = Test2 {
            a: true,
            b: 8,
            c: 16,
            d: 32,
            e: 8,
            f: 16,
            g: 64,
            h: 32.3333,
            i: 64.4444,
            j: "testing string".to_owned(),
        };
        let inner_struct = Test3 {
            a: -1600,
            b: all_basic
        };
        freeze_dry(inner_struct);
    }


    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct Test4 {
        a : Option<i32>
    }

    #[test]
    fn test_optional_field_some() {
        let optional_field = Test4{
            a : Some(4)
        };
        freeze_dry(optional_field);
    }

    #[test]
    fn test_optional_field_none() {
        let optional_field = Test4{
            a : None
        };
        freeze_dry(optional_field);
    }

    ///
    /// Generic Container
    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct Test5<T : AvroSchema> {
        a: String,
        b: Vec<T>
    }


    #[test]
    fn test_generic_container() {
        let test_generic = Test5::<i32> {
            a : "testing".to_owned(),
            b : vec![0,1,2,3]
        };
        freeze_dry(test_generic);
    }

    // ///
    // /// Special Type coericions  
    // #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    // struct Test6 {
    //     bytes: Vec<u8>
    // }

    // #[test]
    // fn test_type_coercion() {
    //     let type_coercion = Test6 {
    //         bytes: vec![255,0,5],
    //     };
    //     // let bytes_schema = match Test6::get_schema() {
    //     //     Schema::Record { 
    //     //         name,
    //     //         doc,
    //     //         fields,
    //     //         lookup
    //     //      } => {
    //     //         fields[*lookup.get("bytes").unwrap()].clone()
    //     //     }
    //     //     _ => unreachable!()
    //     // };
    //     // assert_eq!(Schema::Bytes, bytes_schema.schema);
    //     println!("{:?}", Test6::get_schema());
    //     freeze_dry(type_coercion);
    // }
}