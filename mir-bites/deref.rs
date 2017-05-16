fn main() {
    let x = &4;
    let y = *x;
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: &i32;                    // "x" in scope 1 at <anon>:2:9: 2:10
        scope 2 {
            let _2: i32;                 // "y" in scope 2 at <anon>:3:9: 3:10
        }
    }
    let mut _3: i32;

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:10
        _1 = promoted0;                  // scope 0 at <anon>:2:13: 2:15
        StorageLive(_2);                 // scope 1 at <anon>:3:9: 3:10
        StorageLive(_3);                 // scope 1 at <anon>:3:13: 3:15
        _3 = (*_1);                      // scope 1 at <anon>:3:13: 3:15
        _2 = _3;                         // scope 1 at <anon>:3:13: 3:15
        StorageDead(_3);                 // scope 1 at <anon>:3:15: 3:15
        _0 = ();                         // scope 2 at <anon>:1:11: 4:2
        StorageDead(_2);                 // scope 1 at <anon>:4:2: 4:2
        StorageDead(_1);                 // scope 0 at <anon>:4:2: 4:2
        return;                          // scope 0 at <anon>:4:2: 4:2
    }
}

promoted0 in main: &i32 = {
    let mut _0: &i32;                    // return pointer
    let mut _1: i32;

    bb0: {
        _1 = const 4i32;                 // scope 0 at <anon>:2:14: 2:15
        _0 = &_1;                        // scope 0 at <anon>:2:13: 2:15
        return;                          // scope 0 at <anon>:2:13: 2:15
    }
}
*/
