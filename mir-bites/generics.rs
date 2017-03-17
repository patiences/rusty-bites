trait Hash {
    fn hash(&self) -> u64;
}

impl Hash for bool {
    fn hash(&self) -> u64 {
        if *self { 0 } else { 1 }
    }
}

impl Hash for i64 {
    fn hash(&self) -> u64 {
        *self as u64
    }
}


fn print_hash<T: Hash>(t: &T) {
    println!("The hash is {}", t.hash())
}

pub fn main() {
    print_hash(&true);      // instantiates T = bool
    print_hash(&12_i64);    // instantiates T = i64
}


// MIR

/*
static print_hash::__STATIC_FMTSTR: &'static [&'static str] = {
    let mut _0: &'static [&'static str]; // return pointer
    let mut _1: &'static [&'static str; 2];
    let mut _2: &'static [&'static str; 2];
    let mut _3: [&'static str; 2];

    bb0: {
        StorageLive(_1);                 // scope 0 at <println macros>:3:18: 3:43
        _3 = [const "The hash is ", const "\n"]; // scope 0 at <println macros>:3:18: 3:43
        _2 = &_3;                        // scope 0 at <println macros>:3:18: 3:43
        _1 = &(*_2);                     // scope 0 at <println macros>:3:18: 3:43
        _0 = _1 as &'static [&'static str] (Unsize); // scope 0 at <println macros>:3:18: 3:43
        StorageDead(_1);                 // scope 0 at <println macros>:3:43: 3:43
        return;                          // scope 0 at <println macros>:3:18: 3:43
    }
}

fn <i64 as Hash>::hash(_1: &i64) -> u64 {
    let mut _0: u64;                     // return pointer
    scope 1 {
        let _2: &i64;                    // "self" in scope 1 at <anon>:12:13: 12:18
    }
    let mut _3: i64;

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:12:13: 12:18
        _2 = _1;                         // scope 0 at <anon>:12:13: 12:18
        StorageLive(_3);                 // scope 1 at <anon>:13:9: 13:14
        _3 = (*_2);                      // scope 1 at <anon>:13:9: 13:14
        _0 = _3 as u64 (Misc);           // scope 1 at <anon>:13:9: 13:21
        StorageDead(_3);                 // scope 1 at <anon>:13:21: 13:21
        StorageDead(_2);                 // scope 0 at <anon>:14:6: 14:6
        return;                          // scope 1 at <anon>:14:6: 14:6
    }
}

fn <bool as Hash>::hash(_1: &bool) -> u64 {
    let mut _0: u64;                     // return pointer
    scope 1 {
        let _2: &bool;                   // "self" in scope 1 at <anon>:6:13: 6:18
    }
    let mut _3: bool;

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:6:13: 6:18
        _2 = _1;                         // scope 0 at <anon>:6:13: 6:18
        StorageLive(_3);                 // scope 1 at <anon>:7:12: 7:17
        _3 = (*_2);                      // scope 1 at <anon>:7:12: 7:17
        switchInt(_3) -> [0u8: bb2, otherwise: bb1]; // scope 1 at <anon>:7:9: 7:34
    }

    bb1: {
        _0 = const 0u64;                 // scope 1 at <anon>:7:20: 7:21
        goto -> bb3;                     // scope 1 at <anon>:7:9: 7:34
    }

    bb2: {
        _0 = const 1u64;                 // scope 1 at <anon>:7:31: 7:32
        goto -> bb3;                     // scope 1 at <anon>:7:9: 7:34
    }

    bb3: {
        StorageDead(_3);                 // scope 1 at <anon>:7:34: 7:34
        StorageDead(_2);                 // scope 0 at <anon>:8:6: 8:6
        return;                          // scope 1 at <anon>:8:6: 8:6
    }
}

fn main() -> () {
    let mut _0: ();                      // return pointer
    let mut _1: ();
    let mut _2: &bool;
    let mut _3: &bool;
    let mut _4: bool;
    let mut _5: ();
    let mut _6: &i64;
    let mut _7: &i64;
    let mut _8: i64;

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:23:16: 23:21
        StorageLive(_3);                 // scope 0 at <anon>:23:16: 23:21
        _3 = promoted1;                  // scope 0 at <anon>:23:16: 23:21
        _2 = &(*_3);                     // scope 0 at <anon>:23:16: 23:21
        _1 = print_hash::<bool>(_2) -> bb1; // scope 0 at <anon>:23:5: 23:22
    }

    bb1: {
        StorageDead(_2);                 // scope 0 at <anon>:23:22: 23:22
        StorageDead(_3);                 // scope 0 at <anon>:23:23: 23:23
        StorageLive(_6);                 // scope 0 at <anon>:24:16: 24:23
        StorageLive(_7);                 // scope 0 at <anon>:24:16: 24:23
        _7 = promoted0;                  // scope 0 at <anon>:24:16: 24:23
        _6 = &(*_7);                     // scope 0 at <anon>:24:16: 24:23
        _5 = print_hash::<i64>(_6) -> bb2; // scope 0 at <anon>:24:5: 24:24
    }

    bb2: {
        StorageDead(_6);                 // scope 0 at <anon>:24:24: 24:24
        StorageDead(_7);                 // scope 0 at <anon>:24:25: 24:25
        _0 = ();                         // scope 0 at <anon>:22:15: 25:2
        return;                          // scope 0 at <anon>:25:2: 25:2
    }
}

promoted0 in main: &i64 = {
    let mut _0: &i64;                    // return pointer
    let mut _1: i64;

    bb0: {
        _1 = const 12i64;                // scope 0 at <anon>:24:17: 24:23
        _0 = &_1;                        // scope 0 at <anon>:24:16: 24:23
        return;                          // scope 0 at <anon>:24:16: 24:23
    }
}

promoted1 in main: &bool = {
    let mut _0: &bool;                   // return pointer
    let mut _1: bool;

    bb0: {
        _1 = const true;                 // scope 0 at <anon>:23:17: 23:21
        _0 = &_1;                        // scope 0 at <anon>:23:16: 23:21
        return;                          // scope 0 at <anon>:23:16: 23:21
    }
}

fn print_hash(_1: &T) -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _2: &T;                      // "t" in scope 1 at <anon>:18:24: 18:25
        scope 2 {
            let _13: &u64;               // "__arg0" in scope 2 at <anon>:1:1: 1:1
        }
    }
    let mut _3: std::fmt::Arguments<'_>;
    let mut _4: &[&str];
    let mut _5: &[std::fmt::ArgumentV1<'_>];
    let mut _6: &[std::fmt::ArgumentV1<'_>; 1];
    let mut _7: &[std::fmt::ArgumentV1<'_>; 1];
    let mut _8: [std::fmt::ArgumentV1<'_>; 1];
    let mut _9: (&u64,);
    let mut _10: &u64;
    let mut _11: u64;
    let mut _12: &T;
    let mut _14: std::fmt::ArgumentV1<'_>;
    let mut _15: &u64;
    let mut _16: fn(&u64, &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error>;

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:18:24: 18:25
        _2 = _1;                         // scope 0 at <anon>:18:24: 18:25
        StorageLive(_3);                 // scope 1 at <print macros>:2:27: 2:58
        StorageLive(_4);                 // scope 1 at <println macros>:3:18: 3:43
        _4 = &(*print_hash::__STATIC_FMTSTR); // scope 1 at <println macros>:3:18: 3:43
        StorageLive(_5);                 // scope 1 at <println macros>:3:18: 3:43
        StorageLive(_6);                 // scope 1 at <println macros>:3:18: 3:43
        StorageLive(_7);                 // scope 1 at <println macros>:3:18: 3:43
        StorageLive(_8);                 // scope 1 at <println macros>:3:18: 3:43
        StorageLive(_9);                 // scope 1 at <println macros>:3:18: 3:43
        StorageLive(_10);                // scope 1 at <anon>:19:32: 19:40
        StorageLive(_11);                // scope 1 at <anon>:19:32: 19:40
        StorageLive(_12);                // scope 1 at <anon>:19:32: 19:33
        _12 = &(*_2);                    // scope 1 at <anon>:19:32: 19:33
        _11 = <T as Hash>::hash(_12) -> bb1; // scope 1 at <anon>:19:32: 19:40
    }

    bb1: {
        StorageDead(_12);                // scope 1 at <anon>:19:40: 19:40
        _10 = &_11;                      // scope 1 at <anon>:19:32: 19:40
        _9 = (_10,);                     // scope 1 at <println macros>:3:18: 3:43
        StorageDead(_10);                // scope 1 at <println macros>:3:43: 3:43
        StorageLive(_13);                // scope 1 at <anon>:1:1: 1:1
        _13 = (_9.0: &u64);              // scope 1 at <anon>:1:1: 1:1
        StorageLive(_14);                // scope 2 at <print macros>:2:27: 2:58
        StorageLive(_15);                // scope 2 at <anon>:19:32: 19:40
        _15 = &(*_13);                   // scope 2 at <anon>:19:32: 19:40
        StorageLive(_16);                // scope 2 at <anon>:19:32: 19:40
        _16 = <u64 as std::fmt::Display>::fmt as fn(&u64, &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> (ReifyFnPointer); // scope 2 at <anon>:19:32: 19:40
        _14 = std::fmt::ArgumentV1<'_>::new::<u64>(_15, _16) -> bb2; // scope 2 at <print macros>:2:27: 2:58
    }

    bb2: {
        StorageDead(_16);                // scope 2 at <print macros>:2:58: 2:58
        StorageDead(_15);                // scope 2 at <print macros>:2:58: 2:58
        _8 = [_14];                      // scope 2 at <println macros>:3:18: 3:43
        StorageDead(_14);                // scope 2 at <println macros>:3:43: 3:43
        StorageDead(_13);                // scope 1 at <println macros>:3:43: 3:43
        _7 = &_8;                        // scope 1 at <println macros>:3:18: 3:43
        _6 = &(*_7);                     // scope 1 at <println macros>:3:18: 3:43
        _5 = _6 as &[std::fmt::ArgumentV1<'_>] (Unsize); // scope 1 at <println macros>:3:18: 3:43
        StorageDead(_6);                 // scope 1 at <println macros>:3:43: 3:43
        _3 = std::fmt::Arguments<'_>::new_v1(_4, _5) -> bb3; // scope 1 at <print macros>:2:27: 2:58
    }

    bb3: {
        StorageDead(_5);                 // scope 1 at <print macros>:2:58: 2:58
        StorageDead(_4);                 // scope 1 at <print macros>:2:58: 2:58
        _0 = std::io::_print(_3) -> bb4; // scope 1 at <print macros>:2:1: 2:60
    }

    bb4: {
        StorageDead(_3);                 // scope 1 at <print macros>:2:60: 2:60
        StorageDead(_7);                 // scope 1 at <anon>:20:2: 20:2
        StorageDead(_8);                 // scope 1 at <anon>:20:2: 20:2
        StorageDead(_9);                 // scope 1 at <anon>:20:2: 20:2
        StorageDead(_11);                // scope 1 at <anon>:20:2: 20:2
        StorageDead(_2);                 // scope 0 at <anon>:20:2: 20:2
        return;                          // scope 1 at <anon>:20:2: 20:2
    }
}
*/
