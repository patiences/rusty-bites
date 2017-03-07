fn foo() {

}

fn bar() {

}

pub fn main() {

}

// MIR


/*
fn main() -> () {
    let mut _0: ();                      // return pointer

    bb0: {
        _0 = ();                         // scope 0 at <anon>:9:15: 11:2
        return;                          // scope 0 at <anon>:11:2: 11:2
    }
}

fn bar() -> () {
    let mut _0: ();                      // return pointer

    bb0: {
        _0 = ();                         // scope 0 at <anon>:5:10: 7:2
        return;                          // scope 0 at <anon>:7:2: 7:2
    }
}

fn foo() -> () {
    let mut _0: ();                      // return pointer

    bb0: {
        _0 = ();                         // scope 0 at <anon>:1:10: 3:2
        return;                          // scope 0 at <anon>:3:2: 3:2
    }
}
*/
