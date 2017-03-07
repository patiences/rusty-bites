fn main() {
    let a: i32 = 5; // or let type be inferred and do let a = 5;
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: i32;                     // "a" in scope 1 at <anon>:2:9: 2:10
    }
    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:10
        _1 = const 5i32;                 // scope 0 at <anon>:2:18: 2:19
        _0 = ();                         // scope 1 at <anon>:1:11: 3:2
        StorageDead(_1);                 // scope 0 at <anon>:3:2: 3:2
        return;                          // scope 0 at <anon>:3:2: 3:2
    }
}
*/
