fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let a = vec[3];
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: std::vec::Vec<i32>;      // "vec" in scope 1 at <anon>:2:9: 2:12
        scope 2 {
            let _6: i32;                 // "a" in scope 2 at <anon>:3:9: 3:10
        }
    }
    let mut _2: std::boxed::Box<[i32]>;
    let mut _3: std::boxed::Box<[i32; 5]>;
    let mut _4: std::boxed::Box<[i32; 5]>;
    let mut _5: ();
    let mut _7: i32;
    let mut _8: &i32;
    let mut _9: &std::vec::Vec<i32>;

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:12
        StorageLive(_2);                 // scope 0 at <vec macros>:3:25: 3:46
        StorageLive(_3);                 // scope 0 at <vec macros>:3:25: 3:46
        _4 = Box([i32; 5]);              // scope 0 at <vec macros>:3:25: 3:46
        (*_4) = [const 0i32, const 1i32, const 2i32, const 3i32, const 4i32]; // scope 0 at <vec macros>:3:29: 3:46
        _3 = _4;                         // scope 0 at <vec macros>:3:25: 3:46
        _2 = _3 as std::boxed::Box<[i32]> (Unsize); // scope 0 at <vec macros>:3:25: 3:46
        drop(_3) -> [return: bb3, unwind: bb2]; // scope 0 at <vec macros>:3:46: 3:46
    }

    bb1: {
        resume;                          // scope 0 at <anon>:1:1: 4:2
    }

    bb2: {
        drop(_2) -> bb1;                 // scope 0 at <vec macros>:3:48: 3:48
    }

    bb3: {
        StorageDead(_3);                 // scope 0 at <vec macros>:3:46: 3:46
        _1 = std::slice::<impl [T]><i32>::into_vec(_2) -> [return: bb4, unwind: bb2]; // scope 0 at <vec macros>:3:1: 3:48
    }

    bb4: {
        drop(_2) -> bb5;                 // scope 0 at <vec macros>:3:48: 3:48
    }

    bb5: {
        StorageDead(_2);                 // scope 0 at <vec macros>:3:48: 3:48
        StorageLive(_6);                 // scope 1 at <anon>:3:9: 3:10
        StorageLive(_7);                 // scope 1 at <anon>:3:13: 3:19
        StorageLive(_8);                 // scope 1 at <anon>:3:13: 3:19
        StorageLive(_9);                 // scope 1 at <anon>:3:13: 3:16
        _9 = &_1;                        // scope 1 at <anon>:3:13: 3:16
        _8 = <std::vec::Vec<i32> as std::ops::Index<usize>>::index(_9, const 3usize) -> [return: bb7, unwind: bb6]; // scope 1 at <anon>:3:13: 3:19
    }

    bb6: {
        drop(_1) -> bb1;                 // scope 0 at <anon>:4:2: 4:2
    }

    bb7: {
        _7 = (*_8);                      // scope 1 at <anon>:3:13: 3:19
        _6 = _7;                         // scope 1 at <anon>:3:13: 3:19
        StorageDead(_7);                 // scope 1 at <anon>:3:19: 3:19
        StorageDead(_9);                 // scope 1 at <anon>:3:19: 3:19
        StorageDead(_8);                 // scope 1 at <anon>:3:20: 3:20
        _0 = ();                         // scope 2 at <anon>:1:11: 4:2
        StorageDead(_6);                 // scope 1 at <anon>:4:2: 4:2
        drop(_1) -> bb8;                 // scope 0 at <anon>:4:2: 4:2
    }

    bb8: {
        StorageDead(_1);                 // scope 0 at <anon>:4:2: 4:2
        return;                          // scope 0 at <anon>:4:2: 4:2
    }
}
*/
