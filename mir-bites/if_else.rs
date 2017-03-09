fn main() {
    let mut a : i32;
    if 3 < 4 {
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
    let mut _2: bool;

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:14
        StorageLive(_2);                 // scope 1 at <anon>:3:8: 3:13
        _2 = Lt(const 3i32, const 4i32); // scope 1 at <anon>:3:8: 3:13
        switchInt(_2) -> [0u8: bb2, otherwise: bb1]; // scope 1 at <anon>:3:5: 7:6
    }

    bb1: {
        _1 = const 1i32;                 // scope 1 at <anon>:4:9: 4:14
        _0 = ();                         // scope 1 at <anon>:3:14: 5:6
        goto -> bb3;                     // scope 1 at <anon>:3:5: 7:6
    }

    bb2: {
        _1 = const 0i32;                 // scope 1 at <anon>:6:9: 6:14
        _0 = ();                         // scope 1 at <anon>:5:12: 7:6
        goto -> bb3;                     // scope 1 at <anon>:3:5: 7:6
    }

    bb3: {
        StorageDead(_2);                 // scope 1 at <anon>:7:6: 7:6
        StorageDead(_1);                 // scope 0 at <anon>:8:2: 8:2
        return;                          // scope 0 at <anon>:8:2: 8:2
    }
}
*/
