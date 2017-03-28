struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Traits can provide default method definitions.
    fn talk(&self) -> &'static str {
        "Hello!"
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    // Default trait methods can be overridden.
    fn talk(&self) -> &'static str {
        "Baaah!"
    }
}

fn main() {
    // Type annotation is necessary in this case.
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.is_naked();
}

// MIR

/*
fn <Sheep as Animal>::talk(_1: &Sheep) -> &'static str {
    let mut _0: &'static str;            // return pointer
    scope 1 {
        let _2: &Sheep;                  // "self" in scope 1 at <anon>:28:13: 28:18
    }

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:28:13: 28:18
        _2 = _1;                         // scope 0 at <anon>:28:13: 28:18
        _0 = const "Baaah!";             // scope 1 at <anon>:29:9: 29:17
        StorageDead(_2);                 // scope 0 at <anon>:30:6: 30:6
        return;                          // scope 1 at <anon>:30:6: 30:6
    }
}

fn Sheep::is_naked(_1: &Sheep) -> bool {
    let mut _0: bool;                    // return pointer
    scope 1 {
        let _2: &Sheep;                  // "self" in scope 1 at <anon>:14:17: 14:22
    }
    let mut _3: bool;

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:14:17: 14:22
        _2 = _1;                         // scope 0 at <anon>:14:17: 14:22
        StorageLive(_3);                 // scope 1 at <anon>:15:9: 15:19
        _3 = ((*_2).0: bool);            // scope 1 at <anon>:15:9: 15:19
        _0 = _3;                         // scope 1 at <anon>:15:9: 15:19
        StorageDead(_3);                 // scope 1 at <anon>:15:19: 15:19
        StorageDead(_2);                 // scope 0 at <anon>:16:6: 16:6
        return;                          // scope 1 at <anon>:16:6: 16:6
    }
}

fn <Sheep as Animal>::new(_1: &'static str) -> Sheep {
    let mut _0: Sheep;                   // return pointer
    scope 1 {
        let _2: &'static str;            // "name" in scope 1 at <anon>:23:12: 23:16
    }
    let mut _3: &'static str;

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:23:12: 23:16
        _2 = _1;                         // scope 0 at <anon>:23:12: 23:16
        StorageLive(_3);                 // scope 1 at <anon>:24:23: 24:27
        _3 = _2;                         // scope 1 at <anon>:24:23: 24:27
        _0 = Sheep { naked: const false, name: _3 }; // scope 1 at <anon>:24:9: 24:43
        StorageDead(_3);                 // scope 1 at <anon>:24:43: 24:43
        StorageDead(_2);                 // scope 0 at <anon>:25:6: 25:6
        return;                          // scope 1 at <anon>:25:6: 25:6
    }
}

fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let mut _1: Sheep;               // "dolly" in scope 1 at <anon>:35:9: 35:18
    }
    let mut _2: &'static str;
    let mut _3: &Sheep;
    let mut _4: bool;
    let mut _5: &Sheep;

    bb0: {
        StorageLive(_1);                 // scope 0 at <anon>:35:9: 35:18
        _1 = const Animal::new(const "Dolly") -> bb1; // scope 0 at <anon>:35:28: 35:48
    }

    bb1: {
        StorageLive(_3);                 // scope 1 at <anon>:37:5: 37:10
        _3 = &_1;                        // scope 1 at <anon>:37:5: 37:10
        _2 = const Animal::talk(_3) -> bb2; // scope 1 at <anon>:37:5: 37:17
    }

    bb2: {
        StorageDead(_3);                 // scope 1 at <anon>:37:17: 37:17
        StorageLive(_5);                 // scope 1 at <anon>:38:5: 38:10
        _5 = &_1;                        // scope 1 at <anon>:38:5: 38:10
        _4 = const Sheep::is_naked(_5) -> bb3; // scope 1 at <anon>:38:5: 38:21
    }

    bb3: {
        StorageDead(_5);                 // scope 1 at <anon>:38:21: 38:21
        _0 = ();                         // scope 1 at <anon>:33:11: 39:2
        StorageDead(_1);                 // scope 0 at <anon>:39:2: 39:2
        return;                          // scope 0 at <anon>:39:2: 39:2
    }
}

fn Animal::talk(_1: &Self) -> &'static str {
    let mut _0: &'static str;            // return pointer
    scope 1 {
        let _2: &Self;                   // "self" in scope 1 at <anon>:8:13: 8:18
    }

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:8:13: 8:18
        _2 = _1;                         // scope 0 at <anon>:8:13: 8:18
        _0 = const "Hello!";             // scope 1 at <anon>:9:9: 9:17
        StorageDead(_2);                 // scope 0 at <anon>:10:6: 10:6
        return;                          // scope 1 at <anon>:10:6: 10:6
    }
}
*/
