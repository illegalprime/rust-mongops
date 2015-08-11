use std::collections::BTreeMap;
use std::fmt::{Arguments, Display};

type Object<'a> = BTreeMap<&'a str, SerJson<'a>>;

#[derive(Clone)]
enum SerJson<'a> {
    Map(Object<'a>),
    RawJson(Arguments<'a>),
    I32(i32),
    U32(u32),
    F64(f64),
    Str(&'a str),
    Value(&'a Display),
}

trait Upsert<'a> {
    fn object(&mut self, field: &'a str) -> &mut Self;
}

impl<'a> Upsert<'a> for Object<'a> {
    #[inline]
    fn object(&mut self, field: &'a str) -> &mut Self {
        match self.entry(field).or_insert(SerJson::Map(Object::new())) {
            &mut SerJson::Map(ref mut m) => m,
            _ => unreachable!(),
        }
    }
}

mod update;
pub use update::Update;

#[test]
fn it_works() {
}

