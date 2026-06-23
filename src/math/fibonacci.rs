fn fib(n : u64, modulo : u128) -> u128 {
    fn recur(n : u64, modulo : u128) -> (u128, u128) {
        if n == 0 {
            return (0, 1);
        }

        let (a, b) = recur(n >> 1, modulo);
        let (c, d) = if modulo == 0 {
            let c = a * (2 * b - a);
            let d = a * a + b * b;
            (c, d)
        } else {
            let c = (a * ((2 * b % modulo + modulo - a % modulo) % modulo)) % modulo;
            let d = (a * a % modulo + b * b % modulo) % modulo;
            (c, d)
        };

        if n & 1 == 0 { 
            (c, d)
        } else if modulo == 0 {
            (d, c + d)
        } else {
            (d, (c + d) % modulo)
        }
    }

    recur(n, modulo).0
}