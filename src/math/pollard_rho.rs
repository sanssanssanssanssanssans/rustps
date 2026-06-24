// 개 시발 러스트 좆같네
use crate::math::miller_rabin::{is_prime_u64, M, OddMont};

#[inline(always)]
pub const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

#[inline(always)]
fn is_square(n: u64) -> Option<u64> {
    let s = (n as f64).sqrt() as u64;

    if s * s == n {
        Some(s)
    } else if s < u32::MAX as u64 && (s + 1) * (s + 1) == n {
        Some(s + 1)
    } else {
        None
    }
}

trait PollardRhoOp<T> {
    fn pollard_rho_type(r: T, init: T) -> T;
}

macro_rules! impl_pollard_rho_type {
    ($ty:ident, $large:ident) => {
        impl PollardRhoOp<$ty> for $ty {
            fn pollard_rho_type(r: $ty, init: $ty) -> $ty {
                let rm = OddMont::<$ty>::new(r);

                let mut x0: $ty = 2;
                let mut k = r;

                const STEP: u32 = 1 << 10;

                while k == r {
                    let mut y = M::<$ty> { v: x0 };
                    x0 += 1;
                    k = 1;

                    let mut itr = STEP;

                    while k == 1 && itr < (1 << 21) {
                        let mut g = M::<$ty> { v: 1 };
                        let x = y;

                        for _ in (0..itr).step_by(STEP as usize) {
                            if k != 1 {
                                break;
                            }

                            for _ in 0..STEP {
                                y = rm.redc(
                                    y.v as $large * y.v as $large
                                        + init as $large,
                                );

                                g = rm.mul(
                                    g,
                                    rm.to_mont(x.v.abs_diff(y.v)),
                                );
                            }

                            k = gcd(g.v as u64, r as u64) as $ty;

                            if k == r {
                                k = 1;

                                let mut py = x;

                                for _ in 0..STEP {
                                    py = rm.redc(
                                        py.v as $large
                                            * py.v as $large
                                            + init as $large,
                                    );

                                    k = gcd(
                                        r as u64,
                                        x.v.abs_diff(py.v) as u64,
                                    ) as $ty;

                                    if k != 1 {
                                        break;
                                    }
                                }

                                if k == 1 {
                                    k = r;
                                }
                            }
                        }

                        itr <<= 1;
                    }
                }

                k
            }
        }
    };
}

impl_pollard_rho_type!(u32, u64);
impl_pollard_rho_type!(u64, u128);

#[inline(always)]
fn pollard_rho(r: u64) -> u64 {
    if r < (1u64 << 32) {
        u32::pollard_rho_type(r as u32, 1) as u64
    } else {
        u64::pollard_rho_type(r, 1)
    }
}

pub fn factorize(mut n: u64) -> Vec<u64> {
    assert!(n != 0);

    let mut res = Vec::new();

    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        while n.is_multiple_of(p) {
            res.push(p);
            n /= p;
        }
    }

    if n == 1 {
        return res;
    }

    let mut stk = vec![(n, 1u32)];

    while let Some((x, cnt)) = stk.pop() {
        if x < 1681 || is_prime_u64(x) {
            for _ in 0..cnt {
                res.push(x);
            }
            continue;
        }

        if let Some(s) = is_square(x) {
            stk.push((s, cnt << 1));
            continue;
        }

        let d = pollard_rho(x);

        debug_assert!(1 < d && d < x);
        debug_assert!(x.is_multiple_of(d));

        stk.push((d, cnt));
        stk.push((x / d, cnt));
    }

    res.sort_unstable();
    res
}

pub fn factorize_dedup(n: u64) -> Vec<(u64, u32)> {
    let factors = factorize(n);
    let mut res: Vec<(u64, u32)> = Vec::new();

    for p in factors {
        if let Some(last) = res.last_mut() {
            if last.0 == p {
                last.1 += 1;
                continue;
            }
        }
        res.push((p, 1u32));
    }

    res
}

pub fn divisors(n: u64) -> Vec<u64> {
    let pf = factorize_dedup(n);

    let mut res = vec![1u64];

    for (p, e) in pf {
        let prev_len = res.len();

        let mut mul = 1u64;

        for _ in 0..e {
            mul *= p;

            for i in 0..prev_len {
                res.push(res[i] * mul);
            }
        }
    }

    res.sort_unstable();
    res
}