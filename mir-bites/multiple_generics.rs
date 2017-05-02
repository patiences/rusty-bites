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

fn is_positive_hash<T: Hash>(t: &T) -> bool {
    t.hash() > 0
}

fn are_positive_hashes<T: Hash, U: Hash>(t: &T, u: &U) -> bool {
    is_positive_hash(t) && is_positive_hash(u)
}

pub fn main() {
    are_positive_hashes(&true, &12_i64);
}

// MIR 

/* 
fn main() -> () {
    let mut _0: ();                      // return pointer
    let mut _1: bool;
    let mut _2: &bool;
    let mut _3: &bool;
    let mut _4: bool;
    let mut _5: &i64;
    let mut _6: &i64;
    let mut _7: i64;

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:26:25: 26:30
        StorageLive(_3);                 // scope 0 at <anon>:26:25: 26:30
        _3 = promoted1;                  // scope 0 at <anon>:26:25: 26:30
        _2 = &(*_3);                     // scope 0 at <anon>:26:25: 26:30
        StorageLive(_5);                 // scope 0 at <anon>:26:32: 26:39
        StorageLive(_6);                 // scope 0 at <anon>:26:32: 26:39
        _6 = promoted0;                  // scope 0 at <anon>:26:32: 26:39
        _5 = &(*_6);                     // scope 0 at <anon>:26:32: 26:39
        _1 = const are_positive_hashes(_2, _5) -> bb1; // scope 0 at <anon>:26:5: 26:40
    }

    bb1: {
        StorageDead(_5);                 // scope 0 at <anon>:26:40: 26:40
        StorageDead(_2);                 // scope 0 at <anon>:26:40: 26:40
        StorageDead(_6);                 // scope 0 at <anon>:26:41: 26:41
        StorageDead(_3);                 // scope 0 at <anon>:26:41: 26:41
        _0 = ();                         // scope 0 at <anon>:25:15: 27:2
        return;                          // scope 0 at <anon>:27:2: 27:2
    }
}

promoted0 in main: &i64 = {
    let mut _0: &i64;                    // return pointer
    let mut _1: i64;

    bb0: {
        _1 = const 12i64;                // scope 0 at <anon>:26:33: 26:39
        _0 = &_1;                        // scope 0 at <anon>:26:32: 26:39
        return;                          // scope 0 at <anon>:26:32: 26:39
    }
}

promoted1 in main: &bool = {
    let mut _0: &bool;                   // return pointer
    let mut _1: bool;

    bb0: {
        _1 = const true;                 // scope 0 at <anon>:26:26: 26:30
        _0 = &_1;                        // scope 0 at <anon>:26:25: 26:30
        return;                          // scope 0 at <anon>:26:25: 26:30
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

fn are_positive_hashes(_1: &T, _2: &U) -> bool {
    let mut _0: bool;                    // return pointer
    scope 1 {
        let _3: &T;                      // "t" in scope 1 at <anon>:21:42: 21:43
        let _4: &U;                      // "u" in scope 1 at <anon>:21:49: 21:50
    }
    let mut _5: bool;
    let mut _6: &T;
    let mut _7: bool;
    let mut _8: &U;

    bb0: {
        StorageLive(_3);                 // scope 0 at <anon>:21:42: 21:43
        _3 = _1;                         // scope 0 at <anon>:21:42: 21:43
        StorageLive(_4);                 // scope 0 at <anon>:21:49: 21:50
        _4 = _2;                         // scope 0 at <anon>:21:49: 21:50
        StorageLive(_5);                 // scope 1 at <anon>:22:5: 22:24
        StorageLive(_6);                 // scope 1 at <anon>:22:22: 22:23
        _6 = &(*_3);                     // scope 1 at <anon>:22:22: 22:23
        _5 = const is_positive_hash(_6) -> bb5; // scope 1 at <anon>:22:5: 22:24
    }

    bb1: {
        _0 = const true;                 // scope 1 at <anon>:22:5: 22:47
        goto -> bb4;                     // scope 1 at <anon>:22:5: 22:47
    }

    bb2: {
        _0 = const false;                // scope 1 at <anon>:22:5: 22:47
        goto -> bb4;                     // scope 1 at <anon>:22:5: 22:47
    }

    bb3: {
        StorageLive(_7);                 // scope 1 at <anon>:22:28: 22:47
        StorageLive(_8);                 // scope 1 at <anon>:22:45: 22:46
        _8 = &(*_4);                     // scope 1 at <anon>:22:45: 22:46
        _7 = const is_positive_hash(_8) -> bb6; // scope 1 at <anon>:22:28: 22:47
    }

    bb4: {
        StorageDead(_7);                 // scope 1 at <anon>:22:47: 22:47
        StorageDead(_5);                 // scope 1 at <anon>:22:47: 22:47
        StorageDead(_4);                 // scope 0 at <anon>:23:2: 23:2
        StorageDead(_3);                 // scope 0 at <anon>:23:2: 23:2
        return;                          // scope 1 at <anon>:23:2: 23:2
    }

    bb5: {
        StorageDead(_6);                 // scope 1 at <anon>:22:24: 22:24
        switchInt(_5) -> [0u8: bb2, otherwise: bb3]; // scope 1 at <anon>:22:5: 22:47
    }

    bb6: {
        StorageDead(_8);                 // scope 1 at <anon>:22:47: 22:47
        switchInt(_7) -> [0u8: bb2, otherwise: bb1]; // scope 1 at <anon>:22:5: 22:47
    }
}

fn is_positive_hash(_1: &T) -> bool {
    let mut _0: bool;                    // return pointer
    scope 1 {
        let _2: &T;                      // "t" in scope 1 at <anon>:17:30: 17:31
    }
    let mut _3: u64;
    let mut _4: &T;

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:17:30: 17:31
        _2 = _1;                         // scope 0 at <anon>:17:30: 17:31
        StorageLive(_3);                 // scope 1 at <anon>:18:5: 18:13
        StorageLive(_4);                 // scope 1 at <anon>:18:5: 18:6
        _4 = &(*_2);                     // scope 1 at <anon>:18:5: 18:6
        _3 = const Hash::hash(_4) -> bb1; // scope 1 at <anon>:18:5: 18:13
    }

    bb1: {
        StorageDead(_4);                 // scope 1 at <anon>:18:13: 18:13
        _0 = Gt(_3, const 0u64);         // scope 1 at <anon>:18:5: 18:17
        StorageDead(_3);                 // scope 1 at <anon>:18:17: 18:17
        StorageDead(_2);                 // scope 0 at <anon>:19:2: 19:2
        return;                          // scope 1 at <anon>:19:2: 19:2
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
*/
