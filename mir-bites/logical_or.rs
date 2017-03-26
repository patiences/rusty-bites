fn foo(a: i32, b: i32, x: i32) -> bool {
    a < x || x < b
}

pub fn main() {
    foo(0, 10, 5);
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    let mut _1: bool;

    bb0: {
        _1 = foo(const 0i32, const 10i32, const 5i32) -> bb1; // scope 0 at <anon>:6:5: 6:18
    }

    bb1: {
        _0 = ();                         // scope 0 at <anon>:5:15: 7:2
        return;                          // scope 0 at <anon>:7:2: 7:2
    }
}

fn foo(_1: i32, _2: i32, _3: i32) -> bool {
    let mut _0: bool;                    // return pointer
    scope 1 {
        let _4: i32;                     // "a" in scope 1 at <anon>:1:8: 1:9
        let _5: i32;                     // "b" in scope 1 at <anon>:1:16: 1:17
        let _6: i32;                     // "x" in scope 1 at <anon>:1:24: 1:25
    }
    let mut _7: bool;
    let mut _8: i32;
    let mut _9: i32;
    let mut _10: bool;
    let mut _11: i32;
    let mut _12: i32;

    bb0: {
        StorageLive(_4);                 // scope 0 at <anon>:1:8: 1:9
        _4 = _1;                         // scope 0 at <anon>:1:8: 1:9
        StorageLive(_5);                 // scope 0 at <anon>:1:16: 1:17
        _5 = _2;                         // scope 0 at <anon>:1:16: 1:17
        StorageLive(_6);                 // scope 0 at <anon>:1:24: 1:25
        _6 = _3;                         // scope 0 at <anon>:1:24: 1:25
        StorageLive(_7);                 // scope 1 at <anon>:2:5: 2:10
        StorageLive(_8);                 // scope 1 at <anon>:2:5: 2:6
        _8 = _4;                         // scope 1 at <anon>:2:5: 2:6
        StorageLive(_9);                 // scope 1 at <anon>:2:9: 2:10
        _9 = _6;                         // scope 1 at <anon>:2:9: 2:10
        _7 = Lt(_8, _9);                 // scope 1 at <anon>:2:5: 2:10
        StorageDead(_9);                 // scope 1 at <anon>:2:10: 2:10
        StorageDead(_8);                 // scope 1 at <anon>:2:10: 2:10
        switchInt(_7) -> [0u8: bb3, otherwise: bb1]; // scope 1 at <anon>:2:5: 2:19
    }

    bb1: {
        _0 = const true;                 // scope 1 at <anon>:2:5: 2:19
        goto -> bb4;                     // scope 1 at <anon>:2:5: 2:19
    }

    bb2: {
        _0 = const false;                // scope 1 at <anon>:2:5: 2:19
        goto -> bb4;                     // scope 1 at <anon>:2:5: 2:19
    }

    bb3: {
        StorageLive(_10);                // scope 1 at <anon>:2:14: 2:19
        StorageLive(_11);                // scope 1 at <anon>:2:14: 2:15
        _11 = _6;                        // scope 1 at <anon>:2:14: 2:15
        StorageLive(_12);                // scope 1 at <anon>:2:18: 2:19
        _12 = _5;                        // scope 1 at <anon>:2:18: 2:19
        _10 = Lt(_11, _12);              // scope 1 at <anon>:2:14: 2:19
        StorageDead(_12);                // scope 1 at <anon>:2:19: 2:19
        StorageDead(_11);                // scope 1 at <anon>:2:19: 2:19
        switchInt(_10) -> [0u8: bb2, otherwise: bb1]; // scope 1 at <anon>:2:5: 2:19
    }

    bb4: {
        StorageDead(_10);                // scope 1 at <anon>:2:19: 2:19
        StorageDead(_7);                 // scope 1 at <anon>:2:19: 2:19
        StorageDead(_6);                 // scope 0 at <anon>:3:2: 3:2
        StorageDead(_5);                 // scope 0 at <anon>:3:2: 3:2
        StorageDead(_4);                 // scope 0 at <anon>:3:2: 3:2
        return;                          // scope 1 at <anon>:3:2: 3:2
    }
}
*/
