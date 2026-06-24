pub mod miller_rabin;
pub mod pollard_rho;

pub fn fib(n: u64, modulo: u128) -> u128 {
    fn rec(n: u64, m: u128) -> (u128, u128) {
        if n == 0 {
            return (0, 1);
        }

        let (a, b) = rec(n >> 1, m);

        let (c, d) = if m == 0 {
            (
                a * (2 * b - a),
                a * a + b * b,
            )
        } else {
            let c = (a * ((2 * b % m + m - a % m) % m)) % m;
            let d = (a * a % m + b * b % m) % m;
            (c, d)
        };

        if n & 1 == 0 {
            (c, d)
        } else if m == 0 {
            (d, c + d)
        } else {
            (d, (c + d) % m)
        }
    }

    if modulo == 0 {
        assert!(n <= 186, "F(n) does not fit in u128");
    }

    rec(n, modulo).0
}