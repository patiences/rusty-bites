fn main() {
    let tup = (1, 2);
    let first = tup.0;
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: (i32, i32);              // "tup" in scope 1 at <anon>:2:9: 2:12
        scope 2 {
            let _2: i32;                 // "first" in scope 2 at <anon>:3:9:
3:14
        }
    }
    let mut _3: i32;

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:12
        _1 = (const 1i32, const 2i32);   // scope 0 at <anon>:2:15: 2:21
        StorageLive(_2);                 // scope 1 at <anon>:3:9: 3:14
        StorageLive(_3);                 // scope 1 at <anon>:3:17: 3:22
        _3 = (_1.0: i32);                // scope 1 at <anon>:3:17: 3:22
        _2 = _3;                         // scope 1 at <anon>:3:17: 3:22
        StorageDead(_3);                 // scope 1 at <anon>:3:22: 3:22
        _0 = ();                         // scope 2 at <anon>:1:11: 4:2
        StorageDead(_2);                 // scope 1 at <anon>:4:2: 4:2
        StorageDead(_1);                 // scope 0 at <anon>:4:2: 4:2
        return;                          // scope 0 at <anon>:4:2: 4:2
    }
}
*/
