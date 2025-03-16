// TODO: this is an example of an orphan rule violation.
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  a foreign type (`u32`, from `std`).
//  Look at the compiler error to get familiar with what it looks like.
//  Then delete the code below and move on to the next exercise.

// orphan rules helps prevent same type having different implementation for a trait from multiple crates.
// when we call 1.is_even(), it should only have one implementation to work with.

// impl PartialEq for u32 {
//     fn eq(&self, _other: &Self) -> bool {
//         todo!()
//     }
// }
