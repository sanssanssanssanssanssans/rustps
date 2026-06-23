use std::io::{self, Read};

const BUF_SIZE: usize = 1 << 16;

pub struct FastI {
    stdin: io::Stdin,
    buf: [u8; BUF_SIZE],
    idx: usize,
    len: usize,
}

impl FastI {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buf: [0; BUF_SIZE],
            idx: 0,
            len: 0,
        }
    }

    #[inline(always)]
    fn refill(&mut self) -> bool {
        self.len = self.stdin.read(&mut self.buf).unwrap_or(0);
        self.idx = 0;
        self.len > 0
    }

    #[inline(always)]
    fn byte(&mut self) -> Option<u8> {
        if self.idx >= self.len && !self.refill() {
            return None;
        }
        let b = unsafe { *self.buf.get_unchecked(self.idx) };
        self.idx += 1;
        Some(b)
    }

    #[inline(always)]
    fn skip_whitespace(&mut self) -> Option<u8> {
        loop {
            match self.byte() {
                Some(c) if c > b' ' => return Some(c),
                Some(_) => continue,
                None => return None,
            }
        }
    }

    #[inline(always)]
    pub fn i64(&mut self) -> i64 {
        let mut c = self.skip_whitespace().expect("unexpected EOF");
        let mut sign = 1i64;

        if c == b'-' {
            sign = -1;
            c = self.byte().expect("unexpected EOF");
        }

        let mut x = 0i64;
        loop {
            x = x * 10 + (c - b'0') as i64;
            match self.byte() {
                Some(nc) if nc > b' ' => c = nc,
                _ => break,
            }
        }
        x * sign
    }

    #[inline(always)]
    pub fn i32(&mut self) -> i32 {
        self.i64() as i32
    }

    #[inline(always)]
    pub fn usize(&mut self) -> usize {
        let mut c = self.skip_whitespace().expect("unexpected EOF");
        let mut x = 0usize;

        loop {
            x = x * 10 + (c - b'0') as usize;
            match self.byte() {
                Some(nc) if nc > b' ' => c = nc,
                _ => break,
            }
        }
        x
    }

    #[inline(always)]
    pub fn u64(&mut self) -> u64 {
        let mut c = self.skip_whitespace().expect("unexpected EOF");
        let mut x = 0u64;

        loop {
            x = x * 10 + (c - b'0') as u64;
            match self.byte() {
                Some(nc) if nc > b' ' => c = nc,
                _ => break,
            }
        }
        x
    }

    #[inline(always)]
    pub fn bytes(&mut self) -> Vec<u8> {
        let mut out = Vec::new();
        if let Some(mut c) = self.skip_whitespace() {
            loop {
                out.push(c);
                match self.byte() {
                    Some(nc) if nc > b' ' => c = nc,
                    _ => break,
                }
            }
        }
        out
    }

    #[inline(always)]
    pub fn string(&mut self) -> String {
        unsafe { String::from_utf8_unchecked(self.bytes()) }
    }

    #[inline(always)]
    pub fn char(&mut self) -> char {
        self.skip_whitespace().expect("unexpected EOF") as char
    }

    #[inline(always)]
    pub fn f64(&mut self) -> f64 {
        let mut c = self.skip_whitespace().expect("unexpected EOF");

        let mut sign = 1.0;
        if c == b'-' {
            sign = -1.0;
            c = self.byte().expect("unexpected EOF");
        }

        let mut x = 0.0;
        while c.is_ascii_digit() {
            x = x * 10.0 + (c - b'0') as f64;

            match self.byte() {
                Some(nc) => c = nc,
                None => return sign * x,
            }
        }

        if c == b'.' {
            let mut p = 0.1;

            while let Some(nc) = self.byte() {
                c = nc;

                if !c.is_ascii_digit() {
                    break;
                }

                x += (c - b'0') as f64 * p;
                p *= 0.1;
            }
        }

        sign * x
    }

    #[inline(always)]
    pub fn f32(&mut self) -> f32 {
        self.f64() as f32
    }

    #[inline(always)]
    pub fn u128(&mut self) -> u128 {
        let mut c = self.skip_whitespace().expect("unexpected EOF");
        let mut x = 0u128;

        loop {
            x = x * 10 + (c - b'0') as u128;
            match self.byte() {
                Some(nc) if nc > b' ' => c = nc,
                _ => break,
            }
        }

        x
    }

    #[inline(always)]
    pub fn i128(&mut self) -> i128 {
        let mut c = self.skip_whitespace().expect("unexpected EOF");
        let mut sign = 1i128;

        if c == b'-' {
            sign = -1;
            c = self.byte().expect("unexpected EOF");
        }

        let mut x = 0i128;
        loop {
            x = x * 10 + (c - b'0') as i128;
            match self.byte() {
                Some(nc) if nc > b' ' => c = nc,
                _ => break,
            }
        }

        x * sign
    }
}