fn main() {
    let mut result = 17;
    let transform = 2;

    let result = match transform {
        1 => 111, 
        2 => 222,
        _ => result >> 2,
    };
}

// MIR 

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let mut _1: i32;                 // "result" in scope 1 at <anon>:2:9:
2:19
        scope 2 {
            let _2: i32;                 // "transform" in scope 2 at
<anon>:3:9: 3:18
            scope 3 {
                let _3: i32;             // "result" in scope 3 at <anon>:5:9:
5:15
            }
        }
    }
    let mut _4: i32;
    let mut _5: (i32, bool);

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:19
        _1 = const 17i32;                // scope 0 at <anon>:2:22: 2:24
        StorageLive(_2);                 // scope 1 at <anon>:3:9: 3:18
        _2 = const 2i32;                 // scope 1 at <anon>:3:21: 3:22
        StorageLive(_3);                 // scope 2 at <anon>:5:9: 5:15
        switchInt(_2) -> [1i32: bb1, 2i32: bb2, otherwise: bb3]; // scope 2 at
<anon>:6:9: 6:10
    }

    bb1: {
        _3 = const 111i32;               // scope 2 at <anon>:6:14: 6:17
        goto -> bb4;                     // scope 2 at <anon>:5:18: 9:6
    }

    bb2: {
        _3 = const 222i32;               // scope 2 at <anon>:7:14: 7:17
        goto -> bb4;                     // scope 2 at <anon>:5:18: 9:6
    }

    bb3: {
        StorageLive(_4);                 // scope 2 at <anon>:8:14: 8:20
        _4 = _1;                         // scope 2 at <anon>:8:14: 8:20
        _5 = CheckedShr(_4, const 2i32); // scope 2 at <anon>:8:14: 8:25
        assert(!(_5.1: bool), "attempt to shift right with overflow") -> bb5; //
scope 2 at <anon>:8:14: 8:25
    }

    bb4: {
        _0 = ();                         // scope 3 at <anon>:1:11: 10:2
        StorageDead(_3);                 // scope 2 at <anon>:10:2: 10:2
        StorageDead(_2);                 // scope 1 at <anon>:10:2: 10:2
        StorageDead(_1);                 // scope 0 at <anon>:10:2: 10:2
        return;                          // scope 0 at <anon>:10:2: 10:2
    }

    bb5: {
        _3 = (_5.0: i32);                // scope 2 at <anon>:8:14: 8:25
        StorageDead(_4);                 // scope 2 at <anon>:8:25: 8:25
        goto -> bb4;                     // scope 2 at <anon>:5:18: 9:6
    }
}
*/
