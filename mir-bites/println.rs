fn main() {
    println!("Hello world!");
}

// MIR

/*
static main::__STATIC_FMTSTR: &'static [&'static str] = {
    let mut _0: &'static [&'static str]; // return pointer
    let mut _1: &'static [&'static str; 1];
    let mut _2: &'static [&'static str; 1];
    let mut _3: [&'static str; 1];

    bb0: {
        StorageLive(_1);                 // scope 0 at <println macros>:2:11:
2:36
        _3 = [const "Hello world!\n"];   // scope 0 at <println macros>:2:11:
2:36
        _2 = &_3;                        // scope 0 at <println macros>:2:11:
2:36
        _1 = &(*_2);                     // scope 0 at <println macros>:2:11:
2:36
        _0 = _1 as &'static [&'static str] (Unsize); // scope 0 at <println macros>:2:11: 2:36
        StorageDead(_1);                 // scope 0 at <println macros>:2:36:
2:36
        return;                          // scope 0 at <println macros>:2:11:
2:36
    }
}

fn main() -> () {
    let mut _0: ();                      // return pointer
    let mut _1: ();
    let mut _2: std::fmt::Arguments<'_>;
    let mut _3: &[&str];
    let mut _4: &[std::fmt::ArgumentV1<'_>];
    let mut _5: &[std::fmt::ArgumentV1<'_>; 0];
    let mut _6: &[std::fmt::ArgumentV1<'_>; 0];
    let mut _7: [std::fmt::ArgumentV1<'_>; 0];
    let mut _8: ();

    bb0: {
        StorageLive(_2);                 // scope 0 at <print macros>:2:27: 2:58
        StorageLive(_3);                 // scope 0 at <println macros>:2:11:
2:36
        _3 = &(*main::__STATIC_FMTSTR);  // scope 0 at <println macros>:2:11:
2:36
        StorageLive(_4);                 // scope 0 at <println macros>:2:11:
2:36
        StorageLive(_5);                 // scope 0 at <println macros>:2:11:
2:36
        StorageLive(_6);                 // scope 0 at <println macros>:2:11:
2:36
        StorageLive(_8);                 // scope 0 at <println macros>:2:11:
2:36
        _8 = ();                         // scope 0 at <println macros>:2:11:
2:36
        _6 = promoted0;                  // scope 0 at <println macros>:2:11:
2:36
        _5 = &(*_6);                     // scope 0 at <println macros>:2:11:
2:36
        _4 = _5 as &[std::fmt::ArgumentV1<'_>] (Unsize); // scope 0 at <println macros>:2:11: 2:36
        StorageDead(_5);                 // scope 0 at <println macros>:2:36:
2:36
        _2 = std::fmt::Arguments<'_>::new_v1(_3, _4) -> bb1; // scope 0 at <print macros>:2:27: 2:58
    }

    bb1: {
        StorageDead(_4);                 // scope 0 at <print macros>:2:58: 2:58
        StorageDead(_3);                 // scope 0 at <print macros>:2:58: 2:58
        _1 = std::io::_print(_2) -> bb2; // scope 0 at <print macros>:2:1: 2:60
    }

    bb2: {
        StorageDead(_2);                 // scope 0 at <print macros>:2:60: 2:60
        StorageDead(_6);                 // scope 0 at <print macros>:2:60: 2:60
        StorageDead(_8);                 // scope 0 at <print macros>:2:60: 2:60
        _0 = ();                         // scope 0 at <anon>:1:11: 3:2
        return;                          // scope 0 at <anon>:3:2: 3:2
    }
}

promoted0 in main: &[std::fmt::ArgumentV1<'_>; 0] = {
    let mut _0: &[std::fmt::ArgumentV1<'_>; 0]; // return pointer
    let mut _1: [std::fmt::ArgumentV1<'_>; 0];

    bb0: {
        _1 = [];                         // scope 0 at <println macros>:2:11:
2:36
        _0 = &_1;                        // scope 0 at <println macros>:2:11:
2:36
        return;                          // scope 0 at <println macros>:2:11:
2:36
    }
}
*/
