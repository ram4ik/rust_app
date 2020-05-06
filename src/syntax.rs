pub(crate) fn highest(a: i32, b: u32, c: i8) -> i32 {
    let mut res = a;
    if b as i32 > res {
        res = b as i32;
    }

    if c as i32 > res {
        res = c as i32;
    }

    return res;
}

pub(crate) fn other(a: i32, b: i32) -> i32 {
    let mut c = a + b;
    c = c % 4;
    c = c / 2;
    c += 1;
    // same as return c
    c
}