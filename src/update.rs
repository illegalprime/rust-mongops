use super::{Upsert, Number, Integer};
use super::{Bson, Array, Object};
use std::borrow::Cow;

/* Add easy API for dot notation */

#[derive(Clone)]
pub struct Update<'a>(Object<'a>);

impl<'a> Update<'a> {
    pub fn new() -> Self {
        Update(Object::new())
    }

    pub fn field(&'a mut self, field: &'a str) -> UpdateField {
        UpdateField {
            root:   &mut self.0,
            field:  field,
        }
    }

    pub fn array(&'a mut self, array: &'a str) -> UpdateArray {
        UpdateArray {
            root:   &mut self.0,
            array:  array,
        }
    }

    pub fn isolate(mut self) -> Self {
        self.0.insert(ISOLATED, 1.to_bson_int());
        self
    }

    pub fn no_isolate(mut self) -> Self {
        self.0.remove(ISOLATED);
        self
    }
}

pub struct UpdateField<'a> {
    root:    &'a mut Object<'a>,
    field:   &'a str,
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

    pub fn min(self, value: Bson<'a>) -> Self {
        self.add_modifier(MIN, value)
    }

    pub fn max(self, value: Bson<'a>) -> Self {
        self.add_modifier(MAX, value)
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

    pub fn set(self, value: Bson<'a>) -> Self {
        self.add_modifier(SET, value)
    }

    pub fn unset(self) -> Self {
        self.add_modifier(UNSET, Bson::Str(""))
    }

    pub fn rename(self, name: &'a str) -> Self {
        self.add_modifier(RENAME, Bson::Str(name))
    }

    pub fn set_on_insert(self, value: Bson<'a>) -> Self {
        self.add_modifier(SET_ON_INSERT, value)
    }

    pub fn set_date_now(self) -> Self {
        self.add_modifier(CURRENT_DATE, Bson::Str(DATE_TYPE))
    }

    pub fn set_timestamp_now(self) -> Self {
        self.add_modifier(CURRENT_DATE, Bson::Str(TIMESTAMP_TYPE))
    }

    #[inline]
    fn add_modifier(mut self, category: &'static str, value: Bson<'a>) -> Self {
        {
            let group = self.root.object(category);
            group.insert(self.field, value);
        }
        self
    }

    #[inline]
    fn bit(mut self, op: &'static str, bits: Bson<'a>) -> Self {
        {
	        let bits_update = self.root.object(BIT);
	        let field_ops = bits_update.object(self.field);
	        field_ops.insert(op, bits);
        }
        self
    }
}

pub struct UpdateArray<'a> {
    root:    &'a mut Object<'a>,
    array:   &'a str,
}

impl<'a> UpdateArray<'a> {
    /* TODO: Implement first-matching ( $ symbol )
     * and dot notation for arrays (including index)
     * .first_matching()
     * .index() | These can lead to either a field or another
     * .dot()   | array?
     */
    /* TODO: Refactor for less code bloat */

    pub fn push(self, value: Bson<'a>) -> Self {
        self.add_modifier(PUSH, value)
    }

    #[inline]
    pub fn push_at(self, value: Bson<'a>, position: u32) -> Self {
        self.push_pos_cow(Cow::Owned(vec![value]), position)
    }

    pub fn push_all(self, values: &'a Array<'a>) -> Self {
        {
	        let array = self.root.deep_object(PUSH, self.array);
	        array.insert(EACH, Bson::Array(Cow::Borrowed(values)));
        }
        self
    }

    #[inline]
    pub fn push_all_at(self, values: &'a Array<'a>, position: u32) -> Self {
        self.push_pos_cow(Cow::Borrowed(values), position)
    }

    pub fn slice(self, max: u32) -> Self {
        {
            let array = self.root.deep_object(PUSH, self.array);
            array.entry(EACH).or_insert_with(|| Bson::Array(Cow::Owned(Vec::new())));
            array.insert(SLICE, max.to_bson_int());
        }
        self
    }

    pub fn sort(self) -> Self {
        self.sort_array(1, None)
    }

    pub fn rev_sort(self) -> Self {
        self.sort_array(-1, None)
    }

    pub fn sort_by(self, field: &'a str) -> Self {
        self.sort_array(1, Some(field))
    }

    pub fn rev_sort_by(self, field: &'a str) -> Self {
        self.sort_array(-1, Some(field))
    }

    fn sort_array(mut self, direction: i32, spec: Option<&'a str>) -> Self {
        {
	        let array = self.root.deep_object(PUSH, self.array);
            array.entry(EACH).or_insert_with(|| Bson::Array(Cow::Owned(Vec::new())));
            if let Some(field) = spec {
                let sort = array.object(SORT);
                sort.insert(field, direction.to_bson_int());
            } else {
                array.insert(SORT, direction.to_bson_int());
            }
	    }
        self
    }

    pub fn pull(self, value: Bson<'a>) -> Self {
        self.add_modifier(PULL, value)
    }

    pub fn pull_all(self, values: &'a Array<'a>) -> Self {
        self.add_modifier(PULL_ALL, Bson::Array(Cow::Borrowed(values)))
    }

    /* TODO: Requires Query ops pub fn pull_if(self, condition: ) */

    pub fn pop_front(self) -> Self {
        // TODO: Check if we should be using FloatingPoint()
        // to avoid NumberInt("-1");
        self.add_modifier(POP, (-1).to_bson_int())
    }

    pub fn pop_back(self) -> Self {
        // TODO: Same as above
        self.add_modifier(POP, 1.to_bson_int())
    }

    pub fn add_to_set(self, value: Bson<'a>) -> Self {
        self.add_modifier(ADD_TO_SET, value)
    }

    pub fn add_all_to_set(self, values: &'a Array<'a>) -> Self {
        {
            let array = self.root.deep_object(ADD_TO_SET, self.array);
            array.insert(EACH, Bson::Array(Cow::Borrowed(values)));
        }
        self
    }

    #[inline]
    fn add_modifier(mut self, category: &'static str, value: Bson<'a>) -> Self {
        {
            let group = self.root.object(category);
            group.insert(self.array, value);
        }
        self
    }

    fn push_pos_cow(self, values: Cow<'a, Array<'a>>, position: u32) -> Self {
        {
            let array = self.root.deep_object(PUSH, self.array);
            array.insert(EACH, Bson::Array(values));
            array.insert(POSITION, position.to_bson_int());
        }
        self
    }
}

/*****************
 * Update Fields *
 *****************/
const INCREMENT:      &'static str = "$inc";
const MULTIPLY:       &'static str = "$mul";
const RENAME:         &'static str = "$rename";
const SET_ON_INSERT:  &'static str = "$setOnInsert";
const SET:            &'static str = "$set";
const UNSET:          &'static str = "$unset";
const MIN:            &'static str = "$min";
const MAX:            &'static str = "$max";
const CURRENT_DATE:   &'static str = "$currentDate";
const DATE_TYPE:      &'static str = "{$type:\"date\"}";
const TIMESTAMP_TYPE: &'static str = "{$type:\"timestamp\"}";

/*****************
 * Update Arrays *
 *****************/
/* Operators */
const FIRST:      &'static str = "$";
const ADD_TO_SET: &'static str = "$addToSet";
const POP:        &'static str = "$pop";
const PULL_ALL:   &'static str = "$pullAll";
const PULL:       &'static str = "$pull";
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
