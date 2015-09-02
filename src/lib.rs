/*
 * TODO: Implement ToJson for many things
 * so you can say: push_all(vec![1, 2, 3, 4]);
 */
/* TODO: Make everything CoW */

mod json;
pub use json::{Integer, Number};

mod update;
pub use update::{Update, UpdateField, UpdateArray};

#[test]
fn it_works() {
    let mut update = Update::new();
//    {
//        let mut michael = update.field("michael.score");
//        michael.increment(5).xor(0xfffff);
//    }
    println!("{}", update);
}

