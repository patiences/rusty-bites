fn do_something(v: &Vec<i32>) -> i32 {
    v[0]
}

pub fn main() {
    let v = vec![1, 2, 3];
    do_something(&v);
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: std::vec::Vec<i32>;      // "v" in scope 1 at <anon>:6:9: 6:10
    }
    let mut _2: std::boxed::Box<[i32]>;
    let mut _3: std::boxed::Box<[i32; 3]>;
    let mut _4: std::boxed::Box<[i32; 3]>;
    let mut _5: ();
    let mut _6: i32;
    let mut _7: &std::vec::Vec<i32>;
    let mut _8: &std::vec::Vec<i32>;

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:6:9: 6:10
        StorageLive(_2);                 // scope 0 at <vec macros>:3:25: 3:46
        StorageLive(_3);                 // scope 0 at <vec macros>:3:25: 3:46
        _4 = Box([i32; 3]);              // scope 0 at <vec macros>:3:25: 3:46
        (*_4) = [const 1i32, const 2i32, const 3i32]; // scope 0 at <vec macros>:3:29: 3:46
        _3 = _4;                         // scope 0 at <vec macros>:3:25: 3:46
        _2 = _3 as std::boxed::Box<[i32]> (Unsize); // scope 0 at <vec macros>:3:25: 3:46
        drop(_3) -> [return: bb3, unwind: bb2]; // scope 0 at <vec macros>:3:46: 3:46
    }

    bb1: {
        resume;                          // scope 0 at <anon>:5:1: 8:2
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
        StorageLive(_7);                 // scope 1 at <anon>:7:18: 7:20
        StorageLive(_8);                 // scope 1 at <anon>:7:18: 7:20
        _8 = &_1;                        // scope 1 at <anon>:7:18: 7:20
        _7 = &(*_8);                     // scope 1 at <anon>:7:18: 7:20
        _6 = do_something(_7) -> [return: bb7, unwind: bb6]; // scope 1 at <anon>:7:5: 7:21
    }

    bb6: {
        drop(_1) -> bb1;                 // scope 0 at <anon>:8:2: 8:2
    }

    bb7: {
        StorageDead(_7);                 // scope 1 at <anon>:7:21: 7:21
        StorageDead(_8);                 // scope 1 at <anon>:7:22: 7:22
        _0 = ();                         // scope 1 at <anon>:5:15: 8:2
        drop(_1) -> bb8;                 // scope 0 at <anon>:8:2: 8:2
    }

    bb8: {
        StorageDead(_1);                 // scope 0 at <anon>:8:2: 8:2
        return;                          // scope 0 at <anon>:8:2: 8:2
    }
}

fn do_something(_1: &std::vec::Vec<i32>) -> i32 {
    let mut _0: i32;                     // return pointer
    scope 1 {
        let _2: &std::vec::Vec<i32>;     // "v" in scope 1 at <anon>:1:17: 1:18
    }
    let mut _3: i32;
    let mut _4: &i32;
    let mut _5: &std::vec::Vec<i32>;

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:1:17: 1:18
        _2 = _1;                         // scope 0 at <anon>:1:17: 1:18
        StorageLive(_3);                 // scope 1 at <anon>:2:5: 2:9
        StorageLive(_4);                 // scope 1 at <anon>:2:5: 2:9
        StorageLive(_5);                 // scope 1 at <anon>:2:5: 2:6
        _5 = &(*_2);                     // scope 1 at <anon>:2:5: 2:6
        _4 = <std::vec::Vec<i32> as std::ops::Index<usize>>::index(_5, const 0usize) -> bb1; // scope 1 at <anon>:2:5: 2:9
    }

    bb1: {
        _3 = (*_4);                      // scope 1 at <anon>:2:5: 2:9
        _0 = _3;                         // scope 1 at <anon>:2:5: 2:9
        StorageDead(_3);                 // scope 1 at <anon>:2:9: 2:9
        StorageDead(_5);                 // scope 1 at <anon>:2:9: 2:9
        StorageDead(_4);                 // scope 1 at <anon>:3:2: 3:2
        StorageDead(_2);                 // scope 0 at <anon>:3:2: 3:2
        return;                          // scope 1 at <anon>:3:2: 3:2
    }
}
*/
