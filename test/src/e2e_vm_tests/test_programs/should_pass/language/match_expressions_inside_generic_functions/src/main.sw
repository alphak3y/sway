script;

use std::assert::*;
use std::revert::*;

// fn third_match<Z>(value: Z) -> u8 {
//   match value {
//     foo => 5u8,
//   }
// }

// fn second_match<Y>(value: Y) -> bool {
//   match third_match(value) {
//     1u8 => false,
//     2u8 => false,
//     3u8 => false,
//     4u8 => false,
//     5u8 => true,
//     _ => false,
//   }
// }

// fn first_match<X>(value: X) -> u64 {
//   match second_match(value) {
//     false => 2u64,
//     true => 3u64,
//   }
// }

fn third_if<Q>(value: Q) -> u8 {
  if true {
    5u8
  } else {
    revert(0);
  }
}

fn second_if<R>(value: R) -> bool {
  let third = third_if(value);
  if third == 1u8 || third == 2u8 || third == 3u8 || third == 4u8 {
    false
  } else if third == 5u8 {
    true
  } else {
    false
  }
}

// fn first_if<S>(value: S) -> u64 {
//   let second = second_if(value);
//   if second == false {
//     2u64
//   } else {
//     3u64
//   }
// }

// fn generic_match<C>(value: C) -> u64 {
//   match value {
//     foo => 3u64,
//   }
// }

// fn generic_if<O>(value: O) -> u64 {
//   if true {
//     3u64
//   } else {
//     1u64
//   }
// }

fn main() -> u64 {
  // let a = first_match(true);
  // assert(a == 3);

  // let b = first_match(1u8);
  // assert(b == 3);

  // let c = first_if(true);
  // assert(c == 3);

  // let d = first_if(1u8);
  // assert(d == 3);

  // let e = generic_match(6);
  // assert(e == 3);

  // let f = generic_match(false);
  // assert(f == 3);

  // let g = generic_if(6);
  // assert(g == 3);

  // let h = generic_if(false);
  // assert(h == 3);

  let i = second_if(1u16);
  assert(i);

  let j = second_if(false);
  assert(j);

  1
}