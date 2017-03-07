fn foo() -> i32 {
    1
}

fn main() {
    foo();
}

// MIR

/*
fn main() -> () {
    let mut _0: ();                      // return pointer
    let mut _1: i32;

    bb0: {
        _1 = foo() -> bb1;               // scope 0 at <anon>:6:5: 6:10
    }

    bb1: {
        _0 = ();                         // scope 0 at <anon>:5:11: 7:2
        return;                          // scope 0 at <anon>:7:2: 7:2
    }
}

fn foo() -> i32 {
    let mut _0: i32;                     // return pointer

    bb0: {
        _0 = const 1i32;                 // scope 0 at <anon>:2:5: 2:6
        return;                          // scope 0 at <anon>:3:2: 3:2
    }
}
*/
