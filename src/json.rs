use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::fmt::Error as FmtError;

pub type Object<'a> = BTreeMap<&'a str, Json<'a>>;
pub type Array<'a>  = Vec<Json<'a>>;

#[derive(Clone)]
pub enum Json<'a> {
    Null,
    Bool(bool),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F64(f64),
    Str(&'a str),
    Array(Cow<'a, Array<'a>>),
    Object(Object<'a>),
}

impl<'a> Json<'a> {
    fn as_mut_object(&mut self) -> Option<&mut Object<'a>> {
        match self {
            &mut Json::Object(ref mut o) => Some(o),
            _ => None,
        }
    }
}

/* TODO: Maybe a non-recursive solution? */
impl<'a> Debug for Json<'a> {
    fn fmt(&self, format: &mut Formatter) -> Result<(), FmtError> {
        match self {
            &Json::Null          => format.write_str("null"),
            &Json::Bool(b)       => b.fmt(format),
            &Json::I32(n)        => n.fmt(format),
            &Json::I64(n)        => n.fmt(format),
            &Json::U32(n)        => n.fmt(format),
            &Json::U64(n)        => n.fmt(format),
            &Json::F64(n)        => n.fmt(format),
            &Json::Str(s)        => format.write_fmt(format_args!("\"{}\"", s)),
            &Json::Array(ref a)  => format.debug_list().entries(a.iter()).finish(),
            &Json::Object(ref o) => format.debug_map().entries(o.iter()).finish(),
        }
    }
}

pub trait Upsert<'a> {
    fn object(&mut self, field: &'a str) -> &mut Self;
    fn deep_object(&mut self, op: &'static str, array: &'a str) -> &mut Self;
}

impl<'a> Upsert<'a> for Object<'a> {
    #[inline]
    fn object(&mut self, field: &'a str) -> &mut Self {
        match self.entry(field) {
            Entry::Vacant(v) => v.insert(Json::Object(Object::new())),
            Entry::Occupied(o) => {
                match o.into_mut() {
                    obj @ &mut Json::Object(_) => obj,
                    other @ _ => {
                        *other = Json::Object(Object::new());
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
    fn to_json_int(self) -> Json<'a>;
}

impl<'a> Integer<'a> for i32 {
    #[inline]
    fn to_json_int(self) -> Json<'a> {
        Json::I32(self)
    }
}

impl<'a> Integer<'a> for i64 {
    #[inline]
    fn to_json_int(self) -> Json<'a> {
        Json::I64(self)
    }
}

impl<'a> Integer<'a> for u32 {
    #[inline]
    fn to_json_int(self) -> Json<'a> {
        Json::U32(self)
    }
}

impl<'a> Integer<'a> for u64 {
    #[inline]
    fn to_json_int(self) -> Json<'a> {
        Json::U64(self)
    }
}

pub trait Number<'a> {
    fn to_json_num(self) -> Json<'a>;
}


impl<'a> Number<'a> for f64 {
    #[inline]
    fn to_json_num(self) -> Json<'a> {
        Json::F64(self)
    }
}

impl<'a, I: Integer<'a>> Number<'a> for I {
    #[inline]
    fn to_json_num(self) -> Json<'a> {
        self.to_json_int()
    }
}

