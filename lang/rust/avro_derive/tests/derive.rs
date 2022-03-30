use apache_avro::schema::{AvroSchema, AvroSchemaWithResolved};
use apache_avro::{from_value, Reader, Schema, Writer};
use avro_derive::*;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::collections::HashMap;

#[macro_use]
extern crate serde;

#[cfg(test)]
mod test_derive {
    use super::*;

    /// Takes in a type that implements the right combination of traits and runs it through a Serde Cycle and asserts the result is the same
    fn freeze_dry<T>(obj: T)
    where
        T: std::fmt::Debug + Serialize + DeserializeOwned + AvroSchema + Clone + PartialEq,
    {
        let schema = T::get_schema();
        let mut writer = Writer::new(&schema, Vec::new());
        if let Err(e) = writer.append_ser(obj.clone()) {
            panic!("{}", e.to_string());
        }
        let encoded = writer.into_inner().unwrap();
        let reader = Reader::with_schema(&schema, &encoded[..]).unwrap();
        for res in reader {
            match res {
                Ok(value) => {
                    assert_eq!(obj, from_value::<T>(&value).unwrap());
                }
                Err(e) => panic!("{}", e.to_string()),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct TestBasic {
        a: i32,
        b: String,
    }

    #[test]
    fn test_smoke_test() {
        let test = TestBasic {
            a: 27,
            b: "foo".to_owned(),
        };
        freeze_dry(test);
    }

    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    #[namespace = "com.testing.namespace"]
    struct TestBasicNamesapce {
        a: i32,
        b: String,
    }

    #[test]
    fn test_basic_namesapce() {
        println!("{:?}", TestBasicNamesapce::get_schema())
    }

    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct TestAllSupportedBaseTypes {
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
        j: String,
    }

    #[test]
    fn test_basic_types() {
        let all_basic = TestAllSupportedBaseTypes {
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
    struct TestNested {
        a: i32,
        b: TestAllSupportedBaseTypes,
    }

    #[test]
    fn test_inner_struct() {
        let all_basic = TestAllSupportedBaseTypes {
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
        let inner_struct = TestNested {
            a: -1600,
            b: all_basic,
        };
        freeze_dry(inner_struct);
    }

    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct TestOptional {
        a: Option<i32>,
    }

    #[test]
    fn test_optional_field_some() {
        let optional_field = TestOptional { a: Some(4) };
        freeze_dry(optional_field);
    }

    #[test]
    fn test_optional_field_none() {
        let optional_field = TestOptional { a: None };
        freeze_dry(optional_field);
    }

    /// Generic Containers
    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct TestGeneric<T: AvroSchemaWithResolved> {
        a: String,
        b: Vec<T>,
        c: HashMap<String, T>,
    }

    #[test]
    fn test_generic_container_1() {
        let test_generic = TestGeneric::<i32> {
            a: "testing".to_owned(),
            b: vec![0, 1, 2, 3],
            c: vec![("key".to_owned(), 3)].into_iter().collect(),
        };
        freeze_dry(test_generic);
    }

    #[test]
    fn test_generic_container_2() {
        let test_generic = TestGeneric::<TestAllSupportedBaseTypes> {
            a: "testing".to_owned(),
            b: vec![TestAllSupportedBaseTypes {
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
            }],
            c: vec![(
                "key".to_owned(),
                TestAllSupportedBaseTypes {
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
                },
            )]
            .into_iter()
            .collect(),
        };
        println!(
            "{}",
            TestGeneric::<TestAllSupportedBaseTypes>::get_schema().canonical_form()
        );
        freeze_dry(test_generic);
    }

    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    enum TestAllowedEnum {
        A,
        B,
        C,
        D,
    }

    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct TestAllowedEnumNested {
        a: TestAllowedEnum,
        b: String,
    }

    #[test]
    fn test_enum() {
        let enum_included = TestAllowedEnumNested {
            a: TestAllowedEnum::B,
            b: "hey".to_owned(),
        };
        freeze_dry(enum_included);
    }

    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct ConsList {
        value: i32,
        next: Option<Box<ConsList>>,
    }

    #[test]
    fn test_cons() {
        let list = ConsList {
            value: 34,
            next: Some(Box::new(ConsList {
                value: 42,
                next: None,
            })),
        };
        freeze_dry(list)
    }

    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct ConsListGeneric<T: AvroSchemaWithResolved> {
        value: T,
        next: Option<Box<ConsListGeneric<T>>>,
    }

    #[test]
    fn test_cons_generic() {
        let list = ConsListGeneric::<TestAllowedEnumNested> {
            value: TestAllowedEnumNested {
                a: TestAllowedEnum::B,
                b: "testing".into(),
            },
            next: Some(Box::new(ConsListGeneric::<TestAllowedEnumNested> {
                value: TestAllowedEnumNested {
                    a: TestAllowedEnum::D,
                    b: "testing2".into(),
                },
                next: None,
            })),
        };
        freeze_dry(list)
    }

    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct TestArraysSimple {
        a: [i32; 4],
    }

    #[test]
    fn test_simple_array() {
        let test = TestArraysSimple { a: [2, 3, 4, 5] };
        freeze_dry(test)
    }

    #[derive(Debug, Serialize, Deserialize, AvroSchema, Clone, PartialEq)]
    struct TestComplexArray<T: AvroSchemaWithResolved> {
        a: [T; 2],
    }

    #[test]
    fn test_complex_array() {
        let test = TestComplexArray::<TestBasic> {
            a: [
                TestBasic {
                    a: 27,
                    b: "foo".to_owned(),
                },
                TestBasic {
                    a: 28,
                    b: "bar".to_owned(),
                },
            ],
        };
        freeze_dry(test)
    }
}
