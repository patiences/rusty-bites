fn main() {
    let mut vec : Vec<i32> = Vec::new();
    vec.push(1);
    let first = vec[0];
}

// MIR 

/* 
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let mut _1: std::vec::Vec<i32>;  // "vec" in scope 1 at <anon>:2:9: 2:16
        scope 2 {
            let _5: i32;                 // "first" in scope 2 at <anon>:4:9:
4:14
        }
    }
    let mut _2: ();
    let mut _3: ();
    let mut _4: &mut std::vec::Vec<i32>;
    let mut _6: i32;
    let mut _7: &i32;
    let mut _8: &std::vec::Vec<i32>;

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:16
        _1 = const <std::vec::Vec<T>>::new() -> bb1; // scope 0 at <anon>:2:30:
2:40
    }

    bb1: {
        StorageLive(_4);                 // scope 1 at <anon>:3:5: 3:8
        _4 = &mut _1;                    // scope 1 at <anon>:3:5: 3:8
        _3 = const <std::vec::Vec<T>>::push(_4, const 1i32) -> [return: bb4,
unwind: bb3]; // scope 1 at <anon>:3:5: 3:16
    }

    bb2: {
        resume;                          // scope 0 at <anon>:1:1: 5:2
    }

    bb3: {
        drop(_1) -> bb2;                 // scope 0 at <anon>:5:2: 5:2
    }

    bb4: {
        StorageDead(_4);                 // scope 1 at <anon>:3:16: 3:16
        StorageLive(_5);                 // scope 1 at <anon>:4:9: 4:14
        StorageLive(_6);                 // scope 1 at <anon>:4:17: 4:23
        StorageLive(_7);                 // scope 1 at <anon>:4:17: 4:23
        StorageLive(_8);                 // scope 1 at <anon>:4:17: 4:20
        _8 = &_1;                        // scope 1 at <anon>:4:17: 4:20
        _7 = const std::ops::Index::index(_8, const 0usize) -> [return: bb5,
unwind: bb3]; // scope 1 at <anon>:4:17: 4:23
    }

    bb5: {
        _6 = (*_7);                      // scope 1 at <anon>:4:17: 4:23
        _5 = _6;                         // scope 1 at <anon>:4:17: 4:23
        StorageDead(_6);                 // scope 1 at <anon>:4:23: 4:23
        StorageDead(_8);                 // scope 1 at <anon>:4:23: 4:23
        StorageDead(_7);                 // scope 1 at <anon>:4:24: 4:24
        _0 = ();                         // scope 2 at <anon>:1:11: 5:2
        StorageDead(_5);                 // scope 1 at <anon>:5:2: 5:2
        drop(_1) -> bb6;                 // scope 0 at <anon>:5:2: 5:2
    }

    bb6: {
        StorageDead(_1);                 // scope 0 at <anon>:5:2: 5:2
        return;                          // scope 0 at <anon>:5:2: 5:2
    }
}
*/
