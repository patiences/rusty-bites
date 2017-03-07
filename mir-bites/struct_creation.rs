struct Point {
    x: i32,
    y: i32
}

fn main() {
    let a = Point{ x: 4, y: 5};
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: Point;                   // "a" in scope 1 at <anon>:7:9: 7:10
    }

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:7:9: 7:10
        _1 = Point { x: const 4i32, y: const 5i32 }; // scope 0 at <anon>:7:13:7:31
        _0 = ();                         // scope 1 at <anon>:6:11: 8:2
        StorageDead(_1);                 // scope 0 at <anon>:8:2: 8:2
        return;                          // scope 0 at <anon>:8:2: 8:2
    }
}
*/
