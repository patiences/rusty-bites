fn main() {
    let mut vec : Vec<i32> = Vec::new();
    vec.push(1);
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let mut _1: std::vec::Vec<i32>;  // "vec" in scope 1 at <anon>:2:9: 2:16
    }
    let mut _2: ();
    let mut _3: ();
    let mut _4: &mut std::vec::Vec<i32>;

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:16
        _1 = <std::vec::Vec<T>><i32>::new() -> bb1; // scope 0 at <anon>:2:30:
2:40
    }

    bb1: {
        StorageLive(_4);                 // scope 1 at <anon>:3:5: 3:8
        _4 = &mut _1;                    // scope 1 at <anon>:3:5: 3:8
        _3 = <std::vec::Vec<T>><i32>::push(_4, const 1i32) -> [return: bb4, unwind: bb3]; // scope 1 at <anon>:3:5: 3:16
    }

    bb2: {
        resume;                          // scope 0 at <anon>:1:1: 4:2
    }

    bb3: {
        drop(_1) -> bb2;                 // scope 0 at <anon>:4:2: 4:2
    }

    bb4: {
        StorageDead(_4);                 // scope 1 at <anon>:3:16: 3:16
        _0 = ();                         // scope 1 at <anon>:1:11: 4:2
        drop(_1) -> bb5;                 // scope 0 at <anon>:4:2: 4:2
    }

    bb5: {
        StorageDead(_1);                 // scope 0 at <anon>:4:2: 4:2
        return;                          // scope 0 at <anon>:4:2: 4:2
    }
}
*/
