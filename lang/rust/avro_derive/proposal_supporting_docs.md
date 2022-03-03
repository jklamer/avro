The trait defined within schema.rs
```
pub trait AvroSchema {
    fn get_schema() -> Schema;
}
```

##### Reasoning/Desires 
Associated funtion as the implementation. Not associated const to make schema creation function easier (can use non const functions). The best would be to have this associated function return &'static Schema but I have yet to figure out how to do that without some global state which is undesirable. 

##### Desired user workflow 
Anything that can be serialized the "The serde way" should be able to be serialized/deserialized without further configuration. 

Current Flow
```
use apache_avro::Schema;

let raw_schema = r#"
    {
        "type": "record",
        "name": "test",
        "fields": [
            {"name": "a", "type": "long", "default": 42},
            {"name": "b", "type": "string"}
        ]
    }
"#;

// if the schema is not valid, this function will return an error
let schema = Schema::parse_str(raw_schema).unwrap();

use apache_avro::Writer;

#[derive(Debug, Serialize)]
struct Test {
    a: i64,
    b: String,
}

let mut writer = Writer::new(&schema, Vec::new());
let test = Test {
    a: 27,
    b: "foo".to_owned(),
};
writer.append_ser(test).unwrap();
let encoded = writer.into_inner();
```

New Flow
```
use apache_avro::Writer;

#[derive(Debug, Serialize, AvroSchema)]
struct Test {
    a: i64,
    b: String,
}
// derived schema, always valid or code fails to compile with a descriptive message
let schema = Test::get_schema();

// a writer needs a schema and something to write to
let mut writer = Writer::new(&schema, Vec::new());

// the structure models our Record schema
let test = Test {
    a: 27,
    b: "foo".to_owned(),
};

// schema validation happens here
writer.append_ser(test).unwrap();

// this is how to get back the resulting avro bytecode
// this performs a flush operation to make sure data is written, so it can fail
// you can also call `writer.flush()` yourself without consuming the writer
let encoded = writer.into_inner();
```


##### crate inport 
To use this functionality it comes as an optional feature (modeled off serde)

cargo.toml
```
apache-avro = { version = "X.Y.Z", features = ["derive"] }
```