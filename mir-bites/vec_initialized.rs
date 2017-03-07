fn main() {
    let a = vec![0; 5]; // vec of 5 zeros
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: std::vec::Vec<i32>;      // "a" in scope 1 at <anon>:2:9: 2:10
    }
    let mut _2: ();

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:2:9: 2:10
        _1 = std::vec::from_elem::<i32>(const 0i32, const 5usize) -> bb1; // scope 0 at <vec macros>:2:1: 2:45
    }

    bb1: {
        _0 = ();                         // scope 1 at <anon>:1:11: 3:2
        drop(_1) -> bb2;                 // scope 0 at <anon>:3:2: 3:2
    }

    bb2: {
        StorageDead(_1);                 // scope 0 at <anon>:3:2: 3:2
        return;                          // scope 0 at <anon>:3:2: 3:2
    }
}
*/
