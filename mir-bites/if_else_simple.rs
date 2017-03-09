fn main() {
  let mut a : i32;
  if true {
      a = 1;
  } else {
      a = 0;
  }
}

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let mut _1: i32;                 // "a" in scope 1 at <anon>:2:9: 2:14
    }

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:14
        _1 = const 1i32;                 // scope 1 at <anon>:4:9: 4:14
        _0 = ();                         // scope 1 at <anon>:3:13: 5:6
        StorageDead(_1);                 // scope 0 at <anon>:8:2: 8:2
        return;                          // scope 0 at <anon>:8:2: 8:2
    }
}
*/
