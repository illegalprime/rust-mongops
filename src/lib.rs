use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::borrow::Cow;

pub type Object<'a> = BTreeMap<&'a str, Bson<'a>>;
pub type Array<'a>  = Vec<Bson<'a>>;

/*
 * TODO: Implement ToBson for many things
 * so you can say: push_all(vec![1, 2, 3, 4]);
 */
/* TODO: Make everything CoW */

/// Possible BSON value types.
#[derive(Debug, Clone)]
pub enum Bson<'a> {
    FloatingPoint(f64),
    Str(&'a str),
    Array(Cow<'a, Array<'a>>),
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

impl<'a> Bson<'a> {
    fn as_mut_object(&mut self) -> Option<&mut Object<'a>> {
        match self {
            &mut Bson::Object(ref mut o) => Some(o),
            _ => None,
        }
    }
}

trait Upsert<'a> {
    fn object(&mut self, field: &'a str) -> &mut Self;
    fn deep_object(&mut self, op: &'static str, array: &'a str) -> &mut Self;
}

impl<'a> Upsert<'a> for Object<'a> {
    #[inline]
    fn object(&mut self, field: &'a str) -> &mut Self {
        match self.entry(field) {
            Entry::Vacant(v) => v.insert(Bson::Object(Object::new())),
            Entry::Occupied(o) => {
                match o.into_mut() {
                    obj @ &mut Bson::Object(_) => obj,
                    other @ _ => {
                        *other = Bson::Object(Object::new());
                        other
                    },
                }
            }
        }.as_mut_object().unwrap()
    }

    #[inline]
    fn deep_object(&mut self, op: &'static str, array: &'a str) -> &mut Self {
        self.object(op).object(array)
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
pub use update::{Update, UpdateField, UpdateArray};

#[test]
fn it_works() {
}

