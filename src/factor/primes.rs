// TODO: Write docs later

// We're likely going to have to change out the integer types here for
// something more robust (perhaps just some impl?), but for now they shall be
// u64s
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Prime {
    value: u64
}

impl Prime {
    pub fn new_unchecked(p: u64) -> Prime {
        Prime { value: p }
    }

    pub fn value(self) -> u64 {
        self.value
    }
}
