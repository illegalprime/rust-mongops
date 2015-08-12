use std::collections::BTreeMap;

pub type Object<'a> = BTreeMap<&'a str, Bson<'a>>;

/// Possible BSON value types.
#[derive(Debug, Clone)]
pub enum Bson<'a> {
    FloatingPoint(f64),
    Str(&'a str),
//    Array(Array),
//    Document(Document),
    Boolean(bool),
    Null,
//    RegExp(String, String),
//    JavaScriptCode(String),
//    JavaScriptCodeWithScope(String, Document),
    NumberInt(i32),             // NumberInt("123")
    NumberLong(i64),            // NumberLong("123")
    Object(Object<'a>),
//    TimeStamp(i64),
//    Binary(BinarySubtype, Vec<u8>),
//    ObjectId(oid::ObjectId),
//    UtcDatetime(DateTime<UTC>),
}

trait Upsert<'a> {
    fn object(&mut self, field: &'a str) -> &mut Self;
}

impl<'a> Upsert<'a> for Object<'a> {
    #[inline]
    fn object(&mut self, field: &'a str) -> &mut Self {
        match self.entry(field).or_insert(Bson::Object(Object::new())) {
            &mut Bson::Object(ref mut o) => o,
            _ => unreachable!(),
        }
    }
}

pub trait Integer<'a> {
    fn to_bson_int(self) -> Bson<'a>;
}

impl<'a> Integer<'a> for i32 {
    #[inline]
    fn to_bson_int(self) -> Bson<'a> {
        Bson::NumberInt(self)
    }
}

impl<'a> Integer<'a> for i64 {
    #[inline]
    fn to_bson_int(self) -> Bson<'a> {
        Bson::NumberLong(self)
    }
}

impl<'a> Integer<'a> for u32 {
    #[inline]
    fn to_bson_int(self) -> Bson<'a> {
        Bson::NumberLong(self as i64)
    }
}

pub trait Number<'a> {
    fn to_bson_num(self) -> Bson<'a>;
}


impl<'a> Number<'a> for f64 {
    #[inline]
    fn to_bson_num(self) -> Bson<'a> {
        Bson::FloatingPoint(self)
    }
}

impl<'a, I: Integer<'a>> Number<'a> for I {
    #[inline]
    fn to_bson_num(self) -> Bson<'a> {
        self.to_bson_int()
    }
}

mod update;
pub use update::{Update, UpdateField};

#[test]
fn it_works() {
}

