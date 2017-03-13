fn do_something(x: ()) -> () {
    x
}

pub fn main() {
    let x: () = ();
    do_something(x);
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: ();                      // "x" in scope 1 at <anon>:6:9: 6:10
    }
    let mut _2: ();
    let mut _3: ();

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:6:9: 6:10
        _1 = ();                         // scope 0 at <anon>:6:17: 6:19
        StorageLive(_3);                 // scope 1 at <anon>:7:18: 7:19
        _3 = _1;                         // scope 1 at <anon>:7:18: 7:19
        _2 = do_something(_3) -> bb1;    // scope 1 at <anon>:7:5: 7:20
    }

    bb1: {
        StorageDead(_3);                 // scope 1 at <anon>:7:20: 7:20
        _0 = ();                         // scope 1 at <anon>:5:15: 8:2
        StorageDead(_1);                 // scope 0 at <anon>:8:2: 8:2
        return;                          // scope 0 at <anon>:8:2: 8:2
    }
}

fn do_something(_1: ()) -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _2: ();                      // "x" in scope 1 at <anon>:1:17: 1:18
    }
    let mut _3: ();

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:1:17: 1:18
        _2 = _1;                         // scope 0 at <anon>:1:17: 1:18
        StorageLive(_3);                 // scope 1 at <anon>:2:5: 2:6
        _3 = _2;                         // scope 1 at <anon>:2:5: 2:6
        _0 = _3;                         // scope 1 at <anon>:2:5: 2:6
        StorageDead(_3);                 // scope 1 at <anon>:2:6: 2:6
        StorageDead(_2);                 // scope 0 at <anon>:3:2: 3:2
        return;                          // scope 1 at <anon>:3:2: 3:2
    }
}
*/
