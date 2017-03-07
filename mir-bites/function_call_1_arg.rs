fn foo(n: i32) -> i32 {
    n
}

fn main() {
    foo(1);
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    let mut _1: i32;

    bb0: {
        _1 = foo(const 1i32) -> bb1;     // scope 0 at <anon>:6:5: 6:11
    }

    bb1: {
        _0 = ();                         // scope 0 at <anon>:5:11: 7:2
        return;                          // scope 0 at <anon>:7:2: 7:2
    }
}

fn foo(_1: i32) -> i32 {
    let mut _0: i32;                     // return pointer
    scope 1 {
        let _2: i32;                     // "n" in scope 1 at <anon>:1:8: 1:9
    }
    let mut _3: i32;

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:1:8: 1:9
        _2 = _1;                         // scope 0 at <anon>:1:8: 1:9
        StorageLive(_3);                 // scope 1 at <anon>:2:5: 2:6
        _3 = _2;                         // scope 1 at <anon>:2:5: 2:6
        _0 = _3;                         // scope 1 at <anon>:2:5: 2:6
        StorageDead(_3);                 // scope 1 at <anon>:2:6: 2:6
        StorageDead(_2);                 // scope 0 at <anon>:3:2: 3:2
        return;                          // scope 1 at <anon>:3:2: 3:2
    }
}
*/
