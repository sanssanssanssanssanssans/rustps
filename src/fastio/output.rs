use std::io::{self, Write};

const BUF_SIZE: usize = 1 << 16;

pub struct FastO {
    stdout: io::Stdout,
    buf: [u8; BUF_SIZE],
    idx: usize,
}

impl FastO {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
            buf: [0; BUF_SIZE],
            idx: 0,
        }
    }

    #[inline(always)]
    fn push(&mut self, b: u8) {
        if self.idx == BUF_SIZE {
            self.flush();
        }
        unsafe {
            *self.buf.get_unchecked_mut(self.idx) = b;
        }
        self.idx += 1;
    }

    #[inline(always)]
    pub fn flush(&mut self) {
        if self.idx > 0 {
            self.stdout.write_all(&self.buf[..self.idx]).unwrap();
            self.idx = 0;
        }
    }

    #[inline(always)]
    pub fn byte(&mut self, b: u8) {
        self.push(b);
    }

    #[inline(always)]
    pub fn str(&mut self, s: &str) {
        for &b in s.as_bytes() {
            self.push(b);
        }
    }

    #[inline(always)]
    pub fn space(&mut self) {
        self.push(b' ');
    }

    #[inline(always)]
    pub fn newline(&mut self) {
        self.push(b'\n');
    }

    #[inline(always)]
    pub fn i64(&mut self, mut x: i64) {
        if x == 0 {
            self.push(b'0');
            return;
        }

        if x < 0 {
            self.push(b'-');
            x = -x;
        }

        let mut tmp = [0u8; 20];
        let mut len = 0usize;
        while x > 0 {
            tmp[len] = (x % 10) as u8 + b'0';
            len += 1;
            x /= 10;
        }
        while len > 0 {
            len -= 1;
            self.push(tmp[len]);
        }
    }

    #[inline(always)]
    pub fn i32(&mut self, x: i32) {
        self.i64(x as i64);
    }

    #[inline(always)]
    pub fn usize(&mut self, mut x: usize) {
        if x == 0 {
            self.push(b'0');
            return;
        }

        let mut tmp = [0u8; 20];
        let mut len = 0usize;
        while x > 0 {
            tmp[len] = (x % 10) as u8 + b'0';
            len += 1;
            x /= 10;
        }
        while len > 0 {
            len -= 1;
            self.push(tmp[len]);
        }
    }

    #[inline(always)]
    pub fn u64(&mut self, mut x: u64) {
        if x == 0 {
            self.push(b'0');
            return;
        }

        let mut tmp = [0u8; 20];
        let mut len = 0usize;
        while x > 0 {
            tmp[len] = (x % 10) as u8 + b'0';
            len += 1;
            x /= 10;
        }
        while len > 0 {
            len -= 1;
            self.push(tmp[len]);
        }
    }

    #[inline(always)]
    pub fn u32(&mut self, x: u32) {
        self.u64(x as u64);
    }
}

impl Drop for FastO {
    #[inline(always)]
    fn drop(&mut self) {
        self.flush();
    }
}