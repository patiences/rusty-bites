fn main() {
    let a = 1 + 2;
}
// MIR 

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: i32;                     // "a" in scope 1 at <anon>:2:9: 2:10
    }
    let mut _2: (i32, bool);

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:10
        _2 = CheckedAdd(const 1i32, const 2i32); // scope 0 at <anon>:2:13: 2:18
        assert(!(_2.1: bool), "attempt to add with overflow") -> bb1; // scope 0 at <anon>:2:13: 2:18
    }

    bb1: {
        _1 = (_2.0: i32);                // scope 0 at <anon>:2:13: 2:18
        _0 = ();                         // scope 1 at <anon>:1:11: 3:2
        StorageDead(_1);                 // scope 0 at <anon>:3:2: 3:2
        return;                          // scope 0 at <anon>:3:2: 3:2
    }
}
*/
