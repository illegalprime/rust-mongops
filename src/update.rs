use super::{Object, Upsert, Number, Integer, Bson};

#[derive(Clone)]
pub struct Update<'a> {
    map: Object<'a>,
}

impl<'a> Update<'a> {
    pub fn new() -> Self {
        Update {
            map: Object::new(),
        }
    }

    pub fn field(&'a mut self, field: &'a str) -> UpdateField {
        UpdateField {
            object: self,
            field:  field,
        }
    }
}

pub struct UpdateField<'a> {
    object: &'a mut Update<'a>,
    field:  &'a str,
}

impl<'a> UpdateField<'a> {
    pub fn increment<N: Number<'a>>(self, amount: N) -> Self {
        self.add_modifier(INCREMENT, amount.to_bson_num())
    }

    #[inline]
    pub fn inc<N: Number<'a>>(self, amount: N) -> Self {
        self.increment(amount)
    }

    pub fn multiply<N: Number<'a>>(self, amount: N) -> Self {
        self.add_modifier(MULTIPLY, amount.to_bson_num())
    }

    #[inline]
    pub fn mul<N: Number<'a>>(self, amount: N) -> Self {
        self.multiply(amount)
    }

    pub fn and<I: Integer<'a>>(self, bits: I) -> Self {
        self.bit(AND, bits.to_bson_int())
    }

    pub fn or<I: Integer<'a>>(self, bits: I) -> Self {
        self.bit(OR, bits.to_bson_int())
    }

    pub fn xor<I: Integer<'a>>(self, bits: I) -> Self {
        self.bit(XOR, bits.to_bson_int())
    }

    pub fn rename(self, name: &'a str) -> Self {
        self.add_modifier(RENAME, Bson::Str(name))
    }

    #[inline]
    fn add_modifier(mut self, category: &'static str, value: Bson<'a>) -> Self {
        {
            let group = self.object.map.object(category);
            group.insert(self.field, value);
        }
        self
    }

    #[inline]
    fn bit(mut self, op: &'static str, bits: Bson<'a>) -> Self {
        {
	        let bits_update = self.object.map.object(BIT);
	        let field_ops = bits_update.object(self.field);
	        field_ops.insert(op, bits);
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
