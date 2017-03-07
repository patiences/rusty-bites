fn main() {
    let x_float = 1.0;
    let x_int = x_float as i32;
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: f64;                     // "x_float" in scope 1 at <anon>:2:9:
2:16
        scope 2 {
            let _2: i32;                 // "x_int" in scope 2 at <anon>:3:9:
3:14
        }
    }
    let mut _3: f64;

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:16
        _1 = const F64(1);               // scope 0 at <anon>:2:19: 2:22
        StorageLive(_2);                 // scope 1 at <anon>:3:9: 3:14
        StorageLive(_3);                 // scope 1 at <anon>:3:17: 3:24
        _3 = _1;                         // scope 1 at <anon>:3:17: 3:24
        _2 = _3 as i32 (Misc);           // scope 1 at <anon>:3:17: 3:31
        StorageDead(_3);                 // scope 1 at <anon>:3:31: 3:31
        _0 = ();                         // scope 2 at <anon>:1:11: 4:2
        StorageDead(_2);                 // scope 1 at <anon>:4:2: 4:2
        StorageDead(_1);                 // scope 0 at <anon>:4:2: 4:2
        return;                          // scope 0 at <anon>:4:2: 4:2
    }
}
*/
