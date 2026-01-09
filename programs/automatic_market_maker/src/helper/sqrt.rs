pub fn sqrt(y: u64) -> u64 {
    if y < 4 {
        if y == 0 {
            0
        } else {
            1
        }
    } else {
        let mut z = y;
        let mut x = y / 2 + 1;
        while x < z {
            z = x;
            x = (y / x + x) / 2;
        }
        z
    }
}