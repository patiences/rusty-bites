let x = Box::new(5);
let ptr = Box::into_raw(x);
let x = unsafe { Box::from_raw(ptr) };

// MIR 

/* 
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: std::boxed::Box<i32>;    // "x" in scope 1 at <anon>:2:9: 2:10
        scope 2 {
            let _3: *mut i32;            // "ptr" in scope 2 at <anon>:3:9: 3:12
            scope 3 {
                let _5: std::boxed::Box<i32>; // "x" in scope 3 at <anon>:4:9:
4:10
            }
        }
    }
    let mut _2: ();
    let mut _4: std::boxed::Box<i32>;
    let mut _6: *mut i32;

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:10
        _1 = const <std::boxed::Box<T>>::new(const 5i32) -> bb1; // scope 0 at
<anon>:2:13: 2:24
    }

    bb1: {
        StorageLive(_3);                 // scope 1 at <anon>:3:9: 3:12
        StorageLive(_4);                 // scope 1 at <anon>:3:29: 3:30
        _4 = _1;                         // scope 1 at <anon>:3:29: 3:30
        _3 = const <std::boxed::Box<T>>::into_raw(_4) -> [return: bb5, unwind:
bb4]; // scope 1 at <anon>:3:15: 3:31
    }

    bb2: {
        resume;                          // scope 0 at <anon>:1:1: 6:2
    }

    bb3: {
        drop(_1) -> bb2;                 // scope 0 at <anon>:6:2: 6:2
    }

    bb4: {
        drop(_4) -> bb3;                 // scope 1 at <anon>:3:31: 3:31
    }

    bb5: {
        drop(_4) -> [return: bb6, unwind: bb3]; // scope 1 at <anon>:3:31: 3:31
    }

    bb6: {
        StorageDead(_4);                 // scope 1 at <anon>:3:31: 3:31
        StorageLive(_5);                 // scope 2 at <anon>:4:9: 4:10
        StorageLive(_6);                 // scope 2 at <anon>:4:36: 4:39
        _6 = _3;                         // scope 2 at <anon>:4:36: 4:39
        _5 = const <std::boxed::Box<T>>::from_raw(_6) -> [return: bb7, unwind:
bb3]; // scope 2 at <anon>:4:22: 4:40
    }

    bb7: {
        StorageDead(_6);                 // scope 2 at <anon>:4:40: 4:40
        _0 = ();                         // scope 3 at <anon>:1:11: 6:2
        drop(_5) -> [return: bb8, unwind: bb3]; // scope 2 at <anon>:6:2: 6:2
    }

    bb8: {
        StorageDead(_5);                 // scope 2 at <anon>:6:2: 6:2
        StorageDead(_3);                 // scope 1 at <anon>:6:2: 6:2
        drop(_1) -> bb9;                 // scope 0 at <anon>:6:2: 6:2
    }

    bb9: {
        StorageDead(_1);                 // scope 0 at <anon>:6:2: 6:2
        return;                          // scope 0 at <anon>:6:2: 6:2
    }
}
*/
