use super::SerJson::*;
use super::SerJson;
use super::Object;
use super::Upsert; 

#[derive(Clone)]
pub struct Update<'a>(Object<'a>);

/* TODO: Should *all* bit operations be with u32? */

impl<'a> Update<'a> {
    pub fn new() -> Self {
        Update(Object::new())
    }

    pub fn increment(self, field: &'a str, amount: f64) -> Self {
        self.add_modifier(INCREMENT, field, F64(amount))
    }

    #[inline]
    pub fn inc(self, field: &'a str, amount: f64) -> Self {
        self.increment(field, amount)
    }

    pub fn multiply(self, field: &'a str, amount: f64) -> Self {
        self.add_modifier(MULTIPLY, field, F64(amount))
    }

    #[inline]
    pub fn mul(self, field: &'a str, amount: f64) -> Self {
        self.multiply(field, amount)
    }

    pub fn and(self, field: &'a str, bits: u32) -> Self {
        self.bit(AND, field, bits)
    }

    pub fn or(self, field: &'a str, bits: u32) -> Self {
        self.bit(OR, field, bits)
    }

    pub fn xor(self, field: &'a str, bits: u32) -> Self {
        self.bit(XOR, field, bits)
    }

    pub fn rename(self, field: &'a str, name: &'a str) -> Self {
        self.add_modifier(RENAME, field, Str(name))
    }

    #[inline]
    fn add_modifier(mut self, category: &'static str, field: &'a str, value: SerJson<'a>) -> Self {
        {
            let group = self.0.object(category);
            group.insert(field, value);
        }
        self
    }

    #[inline]
    fn bit(mut self, op: &'static str, field: &'a str, bits: u32) -> Self {
        {
	        let bits_update = self.0.object(BIT);
	        let field_ops = bits_update.object(field);
	        field_ops.insert(op, U32(bits));
        }
        self
    }
}

/*****************
 * Update Fields *
 *****************/
const INCREMENT:     &'static str = "$inc";
const MULTIPLY:      &'static str = "$mul";
const RENAME:        &'static str = "$rename";
const SET_ON_INSERT: &'static str = "$setOnInsert";
const SET:           &'static str = "$set";
const UNSET:         &'static str = "$unset";
const MIN:           &'static str = "$min";
const MAX:           &'static str = "$max";
const CURRENT_DATE:  &'static str = "$currentDate";

/*****************
 * Update Arrays *
 *****************/
/* Operators */
const FIRST:      &'static str = "$";
const ADD_TO_SET: &'static str = "$addToSet";
const POP:        &'static str = "$pop";
const PULL_ALL:   &'static str = "$pullAll";
const PULL:       &'static str = "$pull";
const PUSH_ALL:   &'static str = "$pushAll";
const PUSH:       &'static str = "$push";
/* Modifiers */
const EACH:       &'static str = "$each";
const SLICE:      &'static str = "$slice";
const SORT:       &'static str = "$sort";
const POSITION:   &'static str = "$position";

/*********************
 * Bitwise Operation *
 *********************/
const BIT: &'static str = "$bit";
const AND: &'static str = "and";
const XOR: &'static str = "xor";
const OR:  &'static str = "or";

/*************
 * Isolation *
 *************/
const ISOLATED: &'static str = "$isolated";
