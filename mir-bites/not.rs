fn main() {
    let x = !true;
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: bool;                    // "x" in scope 1 at <anon>:2:9: 2:10
    }

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:10
        _1 = Not(const true);            // scope 0 at <anon>:2:13: 2:18
        _0 = ();                         // scope 1 at <anon>:1:11: 4:2
        StorageDead(_1);                 // scope 0 at <anon>:4:2: 4:2
        return;                          // scope 0 at <anon>:4:2: 4:2
    }
}
*/
