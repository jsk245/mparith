use std::cmp::Ordering;
use std::fmt;
use std::ops;

const B: isize = 1 << (isize::BITS - 2);

#[derive(Debug)]
pub struct BigInt {
    mag: Vec<isize>, // least-significant digit stored in index 0
    sgn: isize,
    len: usize, // https://doc.rust-lang.org/stable/reference/types/numeric.html#machine-dependent-integer-types
}

pub fn build_bigint(val: &str) -> BigInt {
    let first_digit: usize;
    let n: usize = B.ilog(10) as usize;
    let bn: BigInt = BigInt {
        mag: vec![10_isize.pow(n as u32)],
        sgn: 1,
        len: 1,
    };
    let mut res: BigInt = BigInt {
        mag: vec![],
        sgn: 0,
        len: 0,
    };
    let res_sgn: isize;
    match val.chars().next() {
        Some('-') => {
            res_sgn = -1;
            first_digit = 1;
        }
        Some('+') => {
            res_sgn = 1;
            first_digit = 1;
        }
        _ => {
            res_sgn = 1;
            first_digit = 0
        }
    }

    let num_chunks: usize = (val.chars().count() - first_digit) / n;
    let mut chunk: isize;
    let offset: usize = val.chars().count() - n * (num_chunks);
    if first_digit != offset {
        chunk = val[first_digit..offset].parse::<isize>().unwrap();
        if chunk != 0 {
            res = BigInt {
                mag: vec![chunk],
                sgn: 1,
                len: 1,
            };
        }
    }

    for i in 0..num_chunks {
        res = res * &bn;

        chunk = val[(i * n + offset)..((i + 1) * n + offset)]
            .parse::<isize>()
            .unwrap();
        if chunk != 0 {
            res = res
                + BigInt {
                    mag: vec![chunk],
                    sgn: 1,
                    len: 1,
                }
        }
    }

    res.sgn *= res_sgn;
    return res;
}

pub fn build_bigint_bin(val: &str) -> BigInt {
    let n: usize = (isize::BITS - 2) as usize;

    let mut res: BigInt = BigInt {
        mag: vec![],
        sgn: 0,
        len: 0,
    };

    let first_digit: usize;
    match val.chars().next() {
        Some('-') => (first_digit, res.sgn) = (3, -1),
        _ => (first_digit, res.sgn) = (2, 1),
    };

    let num_chunks: usize = (val.chars().count() - first_digit) / n;
    res.len = num_chunks + 1;
    res.mag = vec![0; num_chunks + 1];
    let offset: usize = val.chars().count() - n * (num_chunks);
    if first_digit != offset {
        res.mag[num_chunks] = isize::from_str_radix(&val[first_digit..offset], 2).unwrap();
    }

    for i in 0..num_chunks {
        res.mag[num_chunks - 1 - i] =
            isize::from_str_radix(&val[(i * n + offset)..((i + 1) * n + offset)], 2).unwrap();
    }

    while res.len != 0 && res.mag[res.len - 1] == 0 {
        res.len -= 1
    }

    if res.len == 0 {
        res.sgn = 0;
    }

    return res;
}

fn fmt(val: &BigInt, f: &mut fmt::Formatter) -> fmt::Result {
    let mut s: String = "".to_string();
    let n: usize = B.ilog(10) as usize;
    let bn: BigInt = BigInt {
        mag: vec![10_isize.pow(n as u32)],
        sgn: 1,
        len: 1,
    };
    let mut q: BigInt = val + build_bigint("0");
    if q.sgn == -1 {
        q.sgn = 1;
    }
    let mut r: BigInt;
    let mut r_size: usize;
    let mut r_str: String;

    while q.sgn != 0 {
        (q, r) = divmod(&q, &bn);
        if r.sgn != 0 {
            r_str = r.mag[0].to_string();
            r_size = r_str.chars().count();
            s = r_str + &s;
        } else {
            r_size = 0;
        }
        r_str = "0".repeat(n - r_size);
        s = r_str + &s;
    }

    s = s.trim_start_matches('0').to_string();

    match val.sgn {
        1 => r_str = "".to_string(),
        -1 => r_str = "-".to_string(),
        _ => r_str = "0".to_string(),
    }
    s = r_str + &s;

    return write!(f, "{}", s);
}

impl BigInt {
    pub fn to_string_bin(&self) -> String {
        let mut s: String;
        let mut x: isize;
        let size: usize = (isize::BITS - 2) as usize;
        let mut tmp: String;
        match self.sgn {
            -1 => s = "-0b".to_string(),
            _ => s = "0b".to_string(),
        }
        if self.len == 0 {
            s = s + "0"
        } else {
            x = self.mag[self.len - 1];
            s = s + &String::from(format!("{x:b}"));
        }

        for i in (1..self.len).rev() {
            x = self.mag[i - 1];
            tmp = String::from(format!("{x:b}"));
            s = s + &("0".repeat(size - tmp.chars().count()));
            s = s + &tmp;
        }
        return s;
    }

    fn reduce(&mut self) {
        let mut end = self.len;
        while end > 0 && self.mag[end] == 0 {
            end -= 1;
        }

        if self.mag[end] < 0 {
            for i in 0..=end {
                self.mag[i] = -self.mag[i];
            }
            self.sgn = -self.sgn;
        }

        //TODO: test speedup of reducing base + changing loop structure
        for _ in 0..2 {
            for j in 0..=end {
                if self.mag[j] < 0 {
                    self.mag[j + 1] -= 1;
                    self.mag[j] += B;
                } else if (self.mag[j] & B) != 0 {
                    self.mag[j + 1] += 1;
                    self.mag[j] = self.mag[j] ^ B;
                }
            }
        }

        end += 1;
        while end > 0 && self.mag[end] == 0 {
            end -= 1;
        }

        if end == 0 && self.mag[end] == 0 {
            self.len = 0;
            self.sgn = 0;
            return;
        }
        self.len = end + 1;
    }
}

fn addsub(a: &BigInt, b: &BigInt, sgn: isize) -> BigInt {
    let max: usize;
    let min: usize;
    let a_larger: bool;
    let s: isize = a.sgn * b.sgn * sgn;
    let mut c_mag: Vec<isize>;
    let c_sgn: isize;

    if a.len > b.len {
        max = a.len;
        min = b.len;
        a_larger = true;
    } else {
        max = b.len;
        min = a.len;
        a_larger = false;
    }

    if s == 0 {
        if a.sgn != 0 {
            c_mag = a.mag.clone();
            c_sgn = a.sgn;
        } else {
            c_mag = b.mag.clone();
            c_sgn = b.sgn * sgn;
        }
        return BigInt {
            mag: c_mag,
            sgn: c_sgn,
            len: max,
        };
    } else {
        c_sgn = a.sgn;
    }

    c_mag = Vec::with_capacity(max + 1);

    // TODO: test threading for these 2 loops
    for i in 0..min {
        c_mag.push(a.mag[i] + s * b.mag[i]);
    }
    if a_larger {
        for i in min..max {
            c_mag.push(a.mag[i]);
        }
    } else {
        for i in min..max {
            c_mag.push(s * b.mag[i]);
        }
    }

    c_mag.push(0);

    let mut c: BigInt = BigInt {
        mag: c_mag,
        sgn: c_sgn,
        len: max,
    };
    c.reduce();
    return c;
}

fn mul(a: &BigInt, b: &BigInt) -> BigInt {
    // aa_len <= bb_len
    fn aux(
        aa_mag: Vec<isize>,
        aa_len: usize,
        bb_mag: Vec<isize>,
        bb_len: usize,
        cc_mag: &mut Vec<isize>,
        cc_len: &usize,
    ) {
        let mut s: isize;

        for i in 0..aa_len {
            s = 0;
            for j in 0..=i {
                s = s + aa_mag[j] * bb_mag[i - j];
                if (s & B) != 0 {
                    s = s ^ B;
                    cc_mag[i + 2] += 1;
                }
            }
            cc_mag[i] += s;
        }

        for i in aa_len..bb_len {
            s = 0;
            for j in 0..aa_len {
                s = s + aa_mag[j] * bb_mag[i - j];
                if (s & B) != 0 {
                    s = s ^ B;
                    cc_mag[i + 2] += 1;
                }
            }
            cc_mag[i] += s;
        }

        for i in bb_len..(cc_len - 1) {
            s = 0;
            for j in (i - bb_len + 1)..aa_len {
                s = s + aa_mag[j] * bb_mag[i - j];
                if (s & B) != 0 {
                    s = s ^ B;
                    cc_mag[i + 2] += 1;
                }
            }
            cc_mag[i] += s;
        }
    }

    let c_sgn: isize = a.sgn * b.sgn;
    let mut c_len: usize = a.len + b.len;

    let aa_len: usize = 2 * a.len;
    let mut aa_mag: Vec<isize> = Vec::with_capacity(aa_len);
    let bb_len: usize = 2 * b.len;
    let mut bb_mag: Vec<isize> = Vec::with_capacity(bb_len);
    let cc_len: usize = 2 * c_len;
    let mut cc_mag: Vec<isize> = vec![0; cc_len];

    let lower_bits: isize = isize::MAX >> (isize::BITS / 2);

    if c_sgn == 0 {
        return BigInt {
            mag: vec![],
            sgn: c_sgn,
            len: 0,
        };
    }

    for i in 0..a.len {
        aa_mag.push(a.mag[i] & lower_bits);
        aa_mag.push(a.mag[i] >> (isize::BITS / 2 - 1) & lower_bits);
    }
    for i in 0..b.len {
        bb_mag.push(b.mag[i] & lower_bits);
        bb_mag.push(b.mag[i] >> (isize::BITS / 2 - 1) & lower_bits);
    }

    if aa_len <= bb_len {
        aux(aa_mag, aa_len, bb_mag, bb_len, &mut cc_mag, &cc_len);
    } else {
        aux(bb_mag, bb_len, aa_mag, aa_len, &mut cc_mag, &cc_len);
    }

    for i in 0..(cc_len - 1) {
        cc_mag[i + 1] = cc_mag[i + 1] + (cc_mag[i] >> (isize::BITS / 2 - 1) & lower_bits);
        cc_mag[i] = cc_mag[i] & lower_bits;
    }

    for i in 0..c_len {
        cc_mag[i] = cc_mag[2 * i] + (cc_mag[2 * i + 1] << (isize::BITS / 2 - 1));
    }
    if cc_mag[c_len - 1] == 0 {
        c_len -= 1;
    }

    BigInt {
        mag: cc_mag,
        sgn: c_sgn,
        len: c_len,
    }
}

fn divmod(a: &BigInt, b: &BigInt) -> (BigInt, BigInt) {
    if b.sgn == 0 {
        panic!("Divide by zero error")
    }

    // used in algorithm to keep track of a - bQ, but will be the remainder at the end
    let mut r: BigInt = a + build_bigint("0");
    r.sgn = r.sgn.abs();

    if a.sgn == 0 || a.len < b.len {
        if a.sgn * b.sgn == -1 {
            return (build_bigint("-1"), b - r);
        }
        return (build_bigint("0"), r);
    }

    // for approximation of q
    let B_fp: f64 = B as f64;

    // will be used to store results of division and will eventually be the quotient
    let mut q: BigInt = BigInt {
        mag: vec![0; a.len - b.len + 1],
        sgn: 0,
        len: 0,
    };

    let one: BigInt = build_bigint("1");

    // used to temporaily hold the result of r - bq
    let mut tmp: BigInt = build_bigint("0");

    // for arithmetic involving the factor calculated at each step
    let mut k_bigint: BigInt;

    // for arithmetic involving b scaled by the appropriate factor and base
    let mut bb: BigInt;

    let mut q_len: usize = a.len - b.len + 1;
    let mut ar: f64;
    let br: f64;
    let mut qr: f64;
    let mut s: usize = a.len;
    let mut j: usize;
    let mut k: isize;
    let mut found_factor: bool;

    match b.len {
        1 => br = b.mag[0] as f64,
        _ => br = b.mag[b.len - 1] as f64 + (b.mag[b.len - 2] as f64) / B_fp,
    }

    while s >= b.len {
        if s == b.len {
            match b.sgn {
                1 => tmp = &r - b,
                -1 => tmp = &r + b,
                _ => (),
            }
            if tmp.sgn == -1 {
                break;
            }
        }

        j = r.len - b.len;
        ar = r.mag[s - 1] as f64;
        if s > 1 {
            ar += (r.mag[s - 2] as f64) / B_fp;
        }
        qr = ar / br;
        if qr < 1.0 {
            if j == 0 {
                break;
            }
            qr = qr * B_fp;
            j -= 1;
            if qr < 1.0 {
                qr = 1.0;
            }
        }
        k = qr.floor() as isize;
        k_bigint = build_bigint(&k.to_string());
        q.mag[j] += k;
        found_factor = false;

        while !found_factor {
            bb = &k_bigint * b;
            bb.sgn = 1;
            bb.mag.resize(j + b.len + 1, 0);

            // this scales bb for the subtraction from r since we didn't account for digit place earlier
            for i in (0..=b.len).rev() {
                bb.mag[i + j] = bb.mag[i];
            }
            for i in 0..j {
                bb.mag[i] = 0;
            }
            bb.len += j;

            tmp = &r - bb;
            s = tmp.len;
            if tmp.sgn == -1 {
                k_bigint = k_bigint - &one;
                q.mag[j] -= 1;
                if q.mag[j] == 0 {
                    k_bigint.mag[0] = B - 1;
                    q.mag[j - 1] = B - 1;
                    j -= 1;
                }
            } else {
                for i in 0..s {
                    r.mag[i] = tmp.mag[i];
                }
                r.len = tmp.len;
                r.sgn = tmp.sgn;
                found_factor = true;
            }
        }
    }

    if q.mag[q_len - 1] == 0 {
        q_len -= 1;
    }
    q.sgn = a.sgn * b.sgn;
    q.len = q_len;

    if q_len == 0 {
        r.sgn = a.sgn;
        q.sgn = 0;
    } else {
        r.sgn = a.sgn * b.sgn;
    }

    if a.sgn * b.sgn == -1 && r.len != 0 {
        q = q - one;
        r = r + b;
    }

    if r.len == 0 {
        r.sgn = 0;
    }

    return (q, r);
}

fn shl(a: &BigInt, b: &BigInt) -> BigInt {
    let q: BigInt;
    let r: BigInt;
    // compiler should optimize this but move the build_bigint outside to test later
    (q, r) = divmod(b, &build_bigint(&(isize::BITS - 2).to_string()));

    // compiler should optimize this but move the build_bigint outside to test later
    if q.len > 2 || (q.len == 2 && q.mag[1] > 1) {
        panic!("Integer Overflow");
    }

    let c_mag_leading_zeros: usize;
    match q.len {
        2 => c_mag_leading_zeros = (q.mag[0] | (1 << (isize::BITS - 2))) as usize,
        1 => c_mag_leading_zeros = q.mag[0] as usize,
        _ => c_mag_leading_zeros = 0 as usize,
    }

    let mut c_mag: Vec<isize>;
    let mut c_len: usize = c_mag_leading_zeros + a.len + 1;
    let c_sgn = a.sgn;

    c_mag = vec![0; c_mag_leading_zeros];
    c_mag.extend(&a.mag[0..a.len]);
    c_mag.push(0);

    let mut prev_pushed_val: isize = 0;
    let lower_bits: isize;
    let amount_shifted: u32;
    match r.len {
        1 => {
            lower_bits = isize::MAX >> (1 + r.mag[0]);
            amount_shifted = r.mag[0] as u32;
        }
        _ => {
            lower_bits = isize::MAX >> 1;
            amount_shifted = 0;
        }
    }
    let upper_bits = (isize::MAX >> 1) ^ lower_bits;

    for i in c_mag_leading_zeros..=(a.len + c_mag_leading_zeros) {
        let tmp = c_mag[i] & upper_bits;
        c_mag[i] = prev_pushed_val | ((c_mag[i] & lower_bits) << amount_shifted);
        prev_pushed_val = tmp >> (isize::BITS - 2 - amount_shifted);
    }

    if c_mag[c_len - 1] == 0 {
        c_len -= 1;
    }

    return BigInt {
        mag: c_mag,
        len: c_len,
        sgn: c_sgn,
    };
}

// TODO: see if making a threaded twos complement saves time
fn shr(a: &BigInt, b: &BigInt) -> BigInt {
    let q: BigInt;
    let r: BigInt;
    // compiler should optimize this but move the build_bigint outside to test later
    (q, r) = divmod(b, &build_bigint(&(isize::BITS - 2).to_string()));

    let mut new_first_index: usize = 0;
    let mut big_shift = false;
    let neg_one: BigInt = BigInt {
        mag: vec![1],
        sgn: -1,
        len: 1,
    };

    match q.len {
        0 => new_first_index = 0 as usize,
        1 => new_first_index = q.mag[0] as usize,
        2 => {
            if q.mag[1] > 1 {
                big_shift = true;
            } else {
                new_first_index = (q.mag[0] | (1 << (isize::BITS - 2))) as usize;
            }
        }
        _ => big_shift = true,
    }
    if big_shift || new_first_index >= a.len {
        if a.sgn == -1 {
            return neg_one;
        } else {
            return BigInt {
                mag: vec![],
                sgn: 0,
                len: 0,
            };
        }
    }

    let mut c: BigInt = BigInt {
        mag: a.mag.clone(),
        len: a.len,
        sgn: a.sgn,
    };

    if a.sgn == -1 {
        for i in 0..(c.len) {
            c.mag[i] = c.mag[i] ^ (isize::MAX >> 1);
        }
        c = c + &neg_one;
    }

    c.mag = c.mag[new_first_index..(c.len)].to_vec();
    c.len = c.len - new_first_index;

    let upper_bits: isize;
    let amount_shifted: u32;
    match r.len {
        1 => {
            upper_bits = isize::MAX >> (1 + r.mag[0]) << r.mag[0];
            amount_shifted = r.mag[0] as u32;
        }
        _ => {
            upper_bits = isize::MAX >> 1;
            amount_shifted = 0;
        }
    }
    let lower_bits = (isize::MAX >> 1) ^ upper_bits;

    let mut prev_pushed_val: isize = 0;
    if a.sgn == -1 {
        prev_pushed_val =
            isize::MAX >> (isize::BITS - 1 - amount_shifted) << (isize::BITS - 2 - amount_shifted);
    }
    for i in (0..(c.len)).rev() {
        let tmp = c.mag[i] & lower_bits;
        c.mag[i] = prev_pushed_val | ((c.mag[i] & upper_bits) >> amount_shifted);
        prev_pushed_val = tmp << (isize::BITS - 2 - amount_shifted);
    }

    if a.sgn == -1 {
        for i in 0..(c.len) {
            c.mag[i] = c.mag[i] ^ (isize::MAX >> 1);
        }
        c = c + neg_one;
    } else if c.len > 0 && c.mag[c.len - 1] == 0 {
        c.len -= 1;
        if c.len == 0 {
            c.sgn = 0;
        }
    }

    return c;
}

// TODO: see if making a threaded twos complement saves time
// TODO: see if threading the bitwise and saves time
fn bitand(a: &BigInt, b: &BigInt) -> BigInt {
    if a.len < b.len {
        return bitand(b, a);
    }

    let neg_one: BigInt = BigInt {
        mag: vec![1],
        sgn: -1,
        len: 1,
    };

    let a_mag;
    let mut tmp_a;
    if a.sgn != -1 {
        a_mag = &a.mag;
    } else {
        tmp_a = BigInt {
            mag: a.mag[0..(a.len)].to_vec(),
            sgn: a.sgn,
            len: a.len,
        };
        for i in 0..(a.len) {
            tmp_a.mag[i] = tmp_a.mag[i] ^ (isize::MAX >> 1);
        }
        tmp_a = tmp_a + &neg_one;
        a_mag = &tmp_a.mag;
    }

    let b_mag;
    let mut tmp_b;
    if b.sgn != -1 {
        b_mag = &b.mag;
    } else {
        tmp_b = BigInt {
            mag: b.mag[0..(b.len)].to_vec(),
            sgn: b.sgn,
            len: a.len,
        };
        for i in 0..(b.len) {
            tmp_b.mag[i] = tmp_b.mag[i] ^ (isize::MAX >> 1);
        }
        tmp_b.mag.extend(vec![isize::MAX >> 1; a.len - b.len]);
        tmp_b = tmp_b + &neg_one;
        b_mag = &tmp_b.mag;
    }

    let mut c_sgn = 0;
    match a.sgn * b.sgn {
        1 => c_sgn = a.sgn,
        -1 => c_sgn = 1,
        _ => (),
    }

    let mut c_len;
    if b.sgn == -1 {
        c_len = a.len;
    } else {
        c_len = b.len;
    }

    let mut c_mag = Vec::with_capacity(c_len);

    for i in 0..c_len {
        c_mag.push(a_mag[i] & b_mag[i]);
    }

    let mut c;
    if a.sgn == -1 && b.sgn == -1 {
        for i in 0..c_len {
            c_mag[i] = c_mag[i] ^ (isize::MAX >> 1);
        }
    }

    for i in (0..c_len).rev() {
        if c_mag[i] == 0 {
            c_len -= 1;
        } else {
            break;
        }
    }

    if c_len == 0 {
        c_sgn = 0;
    }

    c = BigInt {
        mag: c_mag,
        len: c_len,
        sgn: c_sgn,
    };

    if a.sgn == -1 && b.sgn == -1 {
        c = c + neg_one;
    }

    return c;
}

// TODO: see if making a threaded twos complement saves time
// TODO: see if threading the bitwise or saves time
fn bitor(a: &BigInt, b: &BigInt) -> BigInt {
    if a.len < b.len {
        return bitor(b, a);
    }

    let neg_one: BigInt = BigInt {
        mag: vec![1],
        sgn: -1,
        len: 1,
    };

    let a_mag;
    let mut tmp_a;
    if a.sgn != -1 {
        a_mag = &a.mag;
    } else {
        tmp_a = BigInt {
            mag: a.mag[0..(a.len)].to_vec(),
            sgn: a.sgn,
            len: a.len,
        };
        for i in 0..(a.len) {
            tmp_a.mag[i] = tmp_a.mag[i] ^ (isize::MAX >> 1);
        }
        tmp_a = tmp_a + &neg_one;
        a_mag = &tmp_a.mag;
    }

    let b_mag;
    let mut tmp_b;
    let b_len;
    if b.sgn != -1 {
        b_mag = &b.mag;
        b_len = b.len;
    } else {
        tmp_b = BigInt {
            mag: b.mag[0..(b.len)].to_vec(),
            sgn: b.sgn,
            len: a.len,
        };
        for i in 0..(b.len) {
            tmp_b.mag[i] = tmp_b.mag[i] ^ (isize::MAX >> 1);
        }
        tmp_b.mag.extend(vec![isize::MAX >> 1; a.len - b.len]);
        tmp_b = tmp_b + &neg_one;
        b_mag = &tmp_b.mag;
        b_len = a.len;
    }

    let mut c_sgn = a.sgn | b.sgn;
    let mut c_len = a.len;

    let mut c_mag = Vec::with_capacity(c_len);
    for i in 0..b_len {
        c_mag.push(a_mag[i] | b_mag[i]);
    }
    for i in b_len..c_len {
        c_mag.push(a_mag[i]);
    }

    let mut c = BigInt {
        mag: c_mag,
        len: c_len,
        sgn: c_sgn,
    };

    if c_sgn == -1 {
        for i in 0..c_len {
            c.mag[i] = c.mag[i] ^ (isize::MAX >> 1);
        }
        c = c + neg_one;
    }

    return c;
}

// TODO: see if making a threaded twos complement saves time
// TODO: see if threading the bitwise xor saves time
fn bitxor(a: &BigInt, b: &BigInt) -> BigInt {
    if a.len < b.len {
        return bitxor(b, a);
    }

    let neg_one: BigInt = BigInt {
        mag: vec![1],
        sgn: -1,
        len: 1,
    };

    let a_mag;
    let mut tmp_a;
    if a.sgn != -1 {
        a_mag = &a.mag;
    } else {
        tmp_a = BigInt {
            mag: a.mag[0..(a.len)].to_vec(),
            sgn: a.sgn,
            len: a.len,
        };
        for i in 0..(a.len) {
            tmp_a.mag[i] = tmp_a.mag[i] ^ (isize::MAX >> 1);
        }
        tmp_a = tmp_a + &neg_one;
        a_mag = &tmp_a.mag;
    }

    let b_mag;
    let mut tmp_b;
    let b_len;
    if b.sgn != -1 {
        b_mag = &b.mag;
        b_len = b.len;
    } else {
        tmp_b = BigInt {
            mag: b.mag[0..(b.len)].to_vec(),
            sgn: b.sgn,
            len: a.len,
        };
        for i in 0..(b.len) {
            tmp_b.mag[i] = tmp_b.mag[i] ^ (isize::MAX >> 1);
        }
        tmp_b.mag.extend(vec![isize::MAX >> 1; a.len - b.len]);
        tmp_b = tmp_b + &neg_one;
        b_mag = &tmp_b.mag;
        b_len = a.len;
    }

    let mut c_sgn;
    match a.sgn ^ b.sgn {
        0 => c_sgn = 1,
        1 => c_sgn = 1,
        _ => c_sgn = -1,
    }
    let mut c_len = a.len;

    let mut c_mag = Vec::with_capacity(c_len);
    for i in 0..b_len {
        c_mag.push(a_mag[i] ^ b_mag[i]);
    }
    for i in b_len..c_len {
        c_mag.push(a_mag[i]);
    }

    let mut c;
    if c_sgn == -1 {
        for i in 0..c_len {
            c_mag[i] = c_mag[i] ^ (isize::MAX >> 1);
        }
    }

    for i in (0..c_len).rev() {
        if c_mag[i] == 0 {
            c_len -= 1;
        } else {
            break;
        }
    }

    let c_sgn_tmp = c_sgn;
    if c_len == 0 {
        c_sgn = 0;
    }

    c = BigInt {
        mag: c_mag,
        len: c_len,
        sgn: c_sgn,
    };

    if c_sgn_tmp == -1 {
        c = c + neg_one;
    }

    return c;
}

fn neg(a: &BigInt) -> BigInt {
    return BigInt {
        mag: a.mag.clone(),
        sgn: -a.sgn,
        len: a.len,
    };
}

impl ops::Add<BigInt> for BigInt {
    type Output = BigInt;

    fn add(self, b: BigInt) -> BigInt {
        addsub(&self, &b, 1)
    }
}

impl ops::Add<&BigInt> for BigInt {
    type Output = BigInt;

    fn add(self, b: &BigInt) -> BigInt {
        addsub(&self, b, 1)
    }
}

impl ops::Add<BigInt> for &BigInt {
    type Output = BigInt;

    fn add(self, b: BigInt) -> BigInt {
        addsub(self, &b, 1)
    }
}

impl ops::Add<&BigInt> for &BigInt {
    type Output = BigInt;

    fn add(self, b: &BigInt) -> BigInt {
        addsub(self, b, 1)
    }
}

impl ops::Sub<BigInt> for BigInt {
    type Output = BigInt;

    fn sub(self, b: BigInt) -> BigInt {
        addsub(&self, &b, -1)
    }
}

impl ops::Sub<&BigInt> for BigInt {
    type Output = BigInt;

    fn sub(self, b: &BigInt) -> BigInt {
        addsub(&self, b, -1)
    }
}

impl ops::Sub<BigInt> for &BigInt {
    type Output = BigInt;

    fn sub(self, b: BigInt) -> BigInt {
        addsub(self, &b, -1)
    }
}

impl ops::Sub<&BigInt> for &BigInt {
    type Output = BigInt;

    fn sub(self, b: &BigInt) -> BigInt {
        addsub(self, b, -1)
    }
}

impl ops::Mul<BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, b: BigInt) -> BigInt {
        mul(&self, &b)
    }
}

impl ops::Mul<&BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, b: &BigInt) -> BigInt {
        mul(&self, b)
    }
}

impl ops::Mul<BigInt> for &BigInt {
    type Output = BigInt;

    fn mul(self, b: BigInt) -> BigInt {
        mul(self, &b)
    }
}

impl ops::Mul<&BigInt> for &BigInt {
    type Output = BigInt;

    fn mul(self, b: &BigInt) -> BigInt {
        mul(self, b)
    }
}

impl ops::Div<BigInt> for BigInt {
    type Output = BigInt;

    fn div(self, b: BigInt) -> BigInt {
        divmod(&self, &b).0
    }
}

impl ops::Div<&BigInt> for BigInt {
    type Output = BigInt;

    fn div(self, b: &BigInt) -> BigInt {
        divmod(&self, b).0
    }
}

impl ops::Div<BigInt> for &BigInt {
    type Output = BigInt;

    fn div(self, b: BigInt) -> BigInt {
        divmod(self, &b).0
    }
}

impl ops::Div<&BigInt> for &BigInt {
    type Output = BigInt;

    fn div(self, b: &BigInt) -> BigInt {
        divmod(self, b).0
    }
}

impl ops::Rem<BigInt> for BigInt {
    type Output = BigInt;

    fn rem(self, b: BigInt) -> BigInt {
        if b.sgn == -1 {
            panic!("Modular arithmetic must be done with positive integers");
        }
        divmod(&self, &b).1
    }
}

impl ops::Rem<&BigInt> for BigInt {
    type Output = BigInt;

    fn rem(self, b: &BigInt) -> BigInt {
        if b.sgn == -1 {
            panic!("Modular arithmetic must be done with positive integers");
        }
        divmod(&self, b).1
    }
}

impl ops::Rem<BigInt> for &BigInt {
    type Output = BigInt;

    fn rem(self, b: BigInt) -> BigInt {
        if b.sgn == -1 {
            panic!("Modular arithmetic must be done with positive integers");
        }
        divmod(self, &b).1
    }
}

impl ops::Rem<&BigInt> for &BigInt {
    type Output = BigInt;

    fn rem(self, b: &BigInt) -> BigInt {
        if b.sgn == -1 {
            panic!("Modular arithmetic must be done with positive integers");
        }
        divmod(self, b).1
    }
}

impl ops::Shl<BigInt> for BigInt {
    type Output = BigInt;

    fn shl(self, b: BigInt) -> BigInt {
        if b.sgn == -1 {
            panic!("Cannot shift by a negative integer");
        }
        shl(&self, &b)
    }
}

impl ops::Shl<&BigInt> for BigInt {
    type Output = BigInt;

    fn shl(self, b: &BigInt) -> BigInt {
        if b.sgn == -1 {
            panic!("Cannot shift by a negative integer");
        }
        shl(&self, b)
    }
}

impl ops::Shl<BigInt> for &BigInt {
    type Output = BigInt;

    fn shl(self, b: BigInt) -> BigInt {
        if b.sgn == -1 {
            panic!("Cannot shift by a negative integer");
        }
        shl(self, &b)
    }
}

impl ops::Shl<&BigInt> for &BigInt {
    type Output = BigInt;

    fn shl(self, b: &BigInt) -> BigInt {
        if b.sgn == -1 {
            panic!("Cannot shift by a negative integer");
        }
        shl(self, b)
    }
}

impl ops::Shr<BigInt> for BigInt {
    type Output = BigInt;

    fn shr(self, b: BigInt) -> BigInt {
        if b.sgn == -1 {
            panic!("Cannot shift by a negative integer");
        }
        shr(&self, &b)
    }
}

impl ops::Shr<&BigInt> for BigInt {
    type Output = BigInt;

    fn shr(self, b: &BigInt) -> BigInt {
        if b.sgn == -1 {
            panic!("Cannot shift by a negative integer");
        }
        shr(&self, b)
    }
}

impl ops::Shr<BigInt> for &BigInt {
    type Output = BigInt;

    fn shr(self, b: BigInt) -> BigInt {
        if b.sgn == -1 {
            panic!("Cannot shift by a negative integer");
        }
        shr(self, &b)
    }
}

impl ops::Shr<&BigInt> for &BigInt {
    type Output = BigInt;

    fn shr(self, b: &BigInt) -> BigInt {
        if b.sgn == -1 {
            panic!("Cannot shift by a negative integer");
        }
        shr(self, b)
    }
}

impl ops::BitAnd<BigInt> for BigInt {
    type Output = BigInt;

    fn bitand(self, b: BigInt) -> BigInt {
        bitand(&self, &b)
    }
}

impl ops::BitAnd<&BigInt> for BigInt {
    type Output = BigInt;

    fn bitand(self, b: &BigInt) -> BigInt {
        bitand(&self, b)
    }
}

impl ops::BitAnd<BigInt> for &BigInt {
    type Output = BigInt;

    fn bitand(self, b: BigInt) -> BigInt {
        bitand(self, &b)
    }
}

impl ops::BitAnd<&BigInt> for &BigInt {
    type Output = BigInt;

    fn bitand(self, b: &BigInt) -> BigInt {
        bitand(self, b)
    }
}

impl ops::BitOr<BigInt> for BigInt {
    type Output = BigInt;

    fn bitor(self, b: BigInt) -> BigInt {
        bitor(&self, &b)
    }
}

impl ops::BitOr<&BigInt> for BigInt {
    type Output = BigInt;

    fn bitor(self, b: &BigInt) -> BigInt {
        bitor(&self, b)
    }
}

impl ops::BitOr<BigInt> for &BigInt {
    type Output = BigInt;

    fn bitor(self, b: BigInt) -> BigInt {
        bitor(self, &b)
    }
}

impl ops::BitOr<&BigInt> for &BigInt {
    type Output = BigInt;

    fn bitor(self, b: &BigInt) -> BigInt {
        bitor(self, b)
    }
}

impl ops::BitXor<BigInt> for BigInt {
    type Output = BigInt;

    fn bitxor(self, b: BigInt) -> BigInt {
        bitxor(&self, &b)
    }
}

impl ops::BitXor<&BigInt> for BigInt {
    type Output = BigInt;

    fn bitxor(self, b: &BigInt) -> BigInt {
        bitxor(&self, b)
    }
}

impl ops::BitXor<BigInt> for &BigInt {
    type Output = BigInt;

    fn bitxor(self, b: BigInt) -> BigInt {
        bitxor(self, &b)
    }
}

impl ops::BitXor<&BigInt> for &BigInt {
    type Output = BigInt;

    fn bitxor(self, b: &BigInt) -> BigInt {
        bitxor(self, b)
    }
}

impl ops::Neg for BigInt {
    type Output = BigInt;

    fn neg(self) -> BigInt {
        neg(&self)
    }
}

impl ops::Neg for &BigInt {
    type Output = BigInt;

    fn neg(self) -> BigInt {
        neg(self)
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt(&self, f)
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.sgn != other.sgn {
            return self.sgn.cmp(&other.sgn);
        }

        if self.len != other.len {
            match self.sgn {
                1 => return self.len.cmp(&other.len),
                -1 => return other.len.cmp(&self.len),
                _ => (),
            }
        }

        for i in (0..self.len).rev() {
            if self.mag[i] != other.mag[i] {
                match self.sgn {
                    1 => return self.mag[i].cmp(&other.mag[i]),
                    -1 => return other.mag[i].cmp(&self.mag[i]),
                    _ => (),
                }
            }
        }

        return Ordering::Equal;
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for BigInt {}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    const A_BIN: usize = 0;
    const A_DEC: usize = 1;
    const B_BIN: usize = 2;
    const B_DEC: usize = 3;
    const SUM_BIN: usize = 4;
    const SUM_DEC: usize = 5;
    const DIFF_BIN: usize = 6;
    const DIFF_DEC: usize = 7;
    const PROD_BIN: usize = 8;
    const PROD_DEC: usize = 9;
    const QUOT_BIN: usize = 10;
    const QUOT_DEC: usize = 11;
    const REM_BIN: usize = 12;
    const REM_DEC: usize = 13;
    const EQ: usize = 14;
    const GT: usize = 15;
    const LT: usize = 16;
    const SHT_AMT_BIN: usize = 17;
    const SHT_AMT_DEC: usize = 18;
    const SHL_BIN: usize = 19;
    const SHL_DEC: usize = 20;
    const SHR_BIN: usize = 21;
    const SHR_DEC: usize = 22;
    const AND_BIN: usize = 23;
    const AND_DEC: usize = 24;
    const OR_BIN: usize = 25;
    const OR_DEC: usize = 26;
    const XOR_BIN: usize = 27;
    const XOR_DEC: usize = 28;
    const A_NEG_BIN: usize = 29;
    const A_NEG_DEC: usize = 30;

    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    #[test]
    fn bigint_bin_io_test() {
        assert_eq!("0b0", super::build_bigint_bin("0b0").to_string_bin());
        assert_eq!("0b0", super::build_bigint_bin("-0b0").to_string_bin());

        if let Ok(lines) = read_lines("./test_inputs.txt") {
            for line in lines {
                if let Ok(testcase) = line {
                    let v: Vec<&str> = testcase.split(',').collect();
                    assert_eq!(v[A_BIN], super::build_bigint_bin(v[A_BIN]).to_string_bin());
                    assert_eq!(v[B_BIN], super::build_bigint_bin(v[B_BIN]).to_string_bin());
                }
            }
        }
    }

    #[test]
    fn bigint_add_test() {
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b0") + super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b1") + super::build_bigint_bin("0b0")).to_string_bin()
        );
        assert_eq!(
            "-0b1",
            (super::build_bigint_bin("0b0") + super::build_bigint_bin("-0b1")).to_string_bin()
        );
        assert_eq!(
            "-0b1",
            (super::build_bigint_bin("-0b1") + super::build_bigint_bin("0b0")).to_string_bin()
        );

        if let Ok(lines) = read_lines("./test_inputs.txt") {
            for line in lines {
                if let Ok(testcase) = line {
                    let v: Vec<&str> = testcase.split(',').collect();
                    assert_eq!(
                        v[SUM_BIN],
                        (super::build_bigint_bin(v[A_BIN]) + super::build_bigint_bin(v[B_BIN]))
                            .to_string_bin()
                    );
                    assert_eq!(
                        v[SUM_BIN],
                        (super::build_bigint(v[A_DEC]) + super::build_bigint(v[B_DEC]))
                            .to_string_bin()
                    );
                    assert_eq!(
                        v[SUM_DEC],
                        (super::build_bigint_bin(v[A_BIN]) + super::build_bigint_bin(v[B_BIN]))
                            .to_string()
                    );
                    assert_eq!(
                        v[SUM_DEC],
                        (super::build_bigint(v[A_DEC]) + super::build_bigint(v[B_DEC])).to_string()
                    );
                }
            }
        }
    }

    #[test]
    fn bigint_sub_test() {
        assert_eq!(
            "-0b1",
            (super::build_bigint_bin("0b0") - super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b1") - super::build_bigint_bin("0b0")).to_string_bin()
        );
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b0") - super::build_bigint_bin("-0b1")).to_string_bin()
        );
        assert_eq!(
            "-0b1",
            (super::build_bigint_bin("-0b1") - super::build_bigint_bin("0b0")).to_string_bin()
        );
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("0b1") - super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("-0b1") - super::build_bigint_bin("-0b1")).to_string_bin()
        );

        if let Ok(lines) = read_lines("./test_inputs.txt") {
            for line in lines {
                if let Ok(testcase) = line {
                    let v: Vec<&str> = testcase.split(',').collect();
                    assert_eq!(
                        v[DIFF_BIN],
                        (super::build_bigint_bin(v[A_BIN]) - super::build_bigint_bin(v[B_BIN]))
                            .to_string_bin()
                    );
                    assert_eq!(
                        v[DIFF_BIN],
                        (super::build_bigint(v[A_DEC]) - super::build_bigint(v[B_DEC]))
                            .to_string_bin()
                    );
                    assert_eq!(
                        v[DIFF_DEC],
                        (super::build_bigint_bin(v[A_BIN]) - super::build_bigint_bin(v[B_BIN]))
                            .to_string()
                    );
                    assert_eq!(
                        v[DIFF_DEC],
                        (super::build_bigint(v[A_DEC]) - super::build_bigint(v[B_DEC])).to_string()
                    );
                }
            }
        }
    }

    #[test]
    fn bigint_mul_test() {
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("0b0") * super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("0b1") * super::build_bigint_bin("0b0")).to_string_bin()
        );
        assert_eq!(
            "-0b1",
            (super::build_bigint_bin("0b1") * super::build_bigint_bin("-0b1")).to_string_bin()
        );
        assert_eq!(
            "-0b1",
            (super::build_bigint_bin("-0b1") * super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b1") * super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("-0b1") * super::build_bigint_bin("-0b1")).to_string_bin()
        );

        if let Ok(lines) = read_lines("./test_inputs.txt") {
            for line in lines {
                if let Ok(testcase) = line {
                    let v: Vec<&str> = testcase.split(',').collect();
                    assert_eq!(
                        v[PROD_BIN],
                        (super::build_bigint_bin(v[A_BIN]) * super::build_bigint_bin(v[B_BIN]))
                            .to_string_bin()
                    );
                    assert_eq!(
                        v[PROD_BIN],
                        (super::build_bigint(v[A_DEC]) * super::build_bigint(v[B_DEC]))
                            .to_string_bin()
                    );
                    assert_eq!(
                        v[PROD_DEC],
                        (super::build_bigint_bin(v[A_BIN]) * super::build_bigint_bin(v[B_BIN]))
                            .to_string()
                    );
                    assert_eq!(
                        v[PROD_DEC],
                        (super::build_bigint(v[A_DEC]) * super::build_bigint(v[B_DEC])).to_string()
                    );
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn bigint_div_byzero_test() {
        super::build_bigint_bin("0b0") / super::build_bigint_bin("0b0");
    }

    #[test]
    fn bigint_div_test() {
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("0b0") / super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("0b0") / super::build_bigint_bin("-0b1")).to_string_bin()
        );

        if let Ok(lines) = read_lines("./test_inputs.txt") {
            for line in lines {
                if let Ok(testcase) = line {
                    let v: Vec<&str> = testcase.split(',').collect();
                    assert_eq!(
                        v[QUOT_BIN],
                        (super::build_bigint_bin(v[A_BIN]) / super::build_bigint_bin(v[B_BIN]))
                            .to_string_bin(),
                    );
                    assert_eq!(
                        v[QUOT_BIN],
                        (super::build_bigint(v[A_DEC]) / super::build_bigint(v[B_DEC]))
                            .to_string_bin()
                    );
                    assert_eq!(
                        v[QUOT_DEC],
                        (super::build_bigint_bin(v[A_BIN]) / super::build_bigint_bin(v[B_BIN]))
                            .to_string()
                    );
                    assert_eq!(
                        v[QUOT_DEC],
                        (super::build_bigint(v[A_DEC]) / super::build_bigint(v[B_DEC])).to_string()
                    );
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn bigint_rem_byzero_test() {
        super::build_bigint_bin("0b0") % super::build_bigint_bin("0b0");
    }

    #[test]
    #[should_panic]
    fn bigint_rem_byneg_test() {
        super::build_bigint_bin("0b0") % super::build_bigint_bin("-0b1");
    }

    #[test]
    fn bigint_rem_test() {
        if let Ok(lines) = read_lines("./test_inputs.txt") {
            for line in lines {
                if let Ok(testcase) = line {
                    let v: Vec<&str> = testcase.split(',').collect();
                    if super::build_bigint_bin(v[B_BIN]).sgn == 1 {
                        assert_eq!(
                            v[REM_BIN],
                            (super::build_bigint_bin(v[A_BIN]) % super::build_bigint_bin(v[B_BIN]))
                                .to_string_bin(),
                        );
                        assert_eq!(
                            v[REM_BIN],
                            (super::build_bigint(v[A_DEC]) % super::build_bigint(v[B_DEC]))
                                .to_string_bin()
                        );
                        assert_eq!(
                            v[REM_DEC],
                            (super::build_bigint_bin(v[A_BIN]) % super::build_bigint_bin(v[B_BIN]))
                                .to_string()
                        );
                        assert_eq!(
                            v[REM_DEC],
                            (super::build_bigint(v[A_DEC]) % super::build_bigint(v[B_DEC]))
                                .to_string()
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn bigint_cmp_test() {
        if let Ok(lines) = read_lines("./test_inputs.txt") {
            for line in lines {
                if let Ok(testcase) = line {
                    let v: Vec<&str> = testcase.split(',').collect();
                    assert_eq!(
                        v[EQ].to_lowercase(),
                        (super::build_bigint_bin(v[A_BIN]) == super::build_bigint_bin(v[B_BIN]))
                            .to_string()
                    );
                    assert_eq!(
                        v[EQ].to_lowercase(),
                        (super::build_bigint(v[A_DEC]) == super::build_bigint(v[B_DEC]))
                            .to_string()
                    );
                    assert_eq!(
                        v[GT].to_lowercase(),
                        (super::build_bigint_bin(v[A_BIN]) > super::build_bigint_bin(v[B_BIN]))
                            .to_string()
                    );
                    assert_eq!(
                        v[GT].to_lowercase(),
                        (super::build_bigint(v[A_DEC]) > super::build_bigint(v[B_DEC])).to_string()
                    );
                    assert_eq!(
                        v[LT].to_lowercase(),
                        (super::build_bigint_bin(v[A_BIN]) < super::build_bigint_bin(v[B_BIN]))
                            .to_string()
                    );
                    assert_eq!(
                        v[LT].to_lowercase(),
                        (super::build_bigint(v[A_DEC]) < super::build_bigint(v[B_DEC])).to_string()
                    );
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn bigint_shl_byneg_test() {
        super::build_bigint_bin("0b0") << super::build_bigint_bin("-0b1");
    }

    #[test]
    fn bigint_shl_test() {
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("0b0") << super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b1") << super::build_bigint_bin("0b0")).to_string_bin()
        );

        if let Ok(lines) = read_lines("./test_inputs.txt") {
            for line in lines {
                if let Ok(testcase) = line {
                    let v: Vec<&str> = testcase.split(',').collect();
                    assert_eq!(
                        v[SHL_BIN],
                        (super::build_bigint_bin(v[A_BIN])
                            << super::build_bigint_bin(v[SHT_AMT_BIN]))
                        .to_string_bin(),
                    );
                    assert_eq!(
                        v[SHL_DEC],
                        (super::build_bigint(v[A_DEC]) << super::build_bigint(v[SHT_AMT_DEC]))
                            .to_string(),
                    );
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn bigint_shr_byneg_test() {
        super::build_bigint_bin("0b0") >> super::build_bigint_bin("-0b1");
    }

    #[test]
    fn bigint_shr_test() {
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("0b0") >> super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b1") >> super::build_bigint_bin("0b0")).to_string_bin()
        );

        if let Ok(lines) = read_lines("./test_inputs.txt") {
            for line in lines {
                if let Ok(testcase) = line {
                    let v: Vec<&str> = testcase.split(',').collect();
                    assert_eq!(
                        v[SHR_BIN],
                        (super::build_bigint_bin(v[A_BIN])
                            >> super::build_bigint_bin(v[SHT_AMT_BIN]))
                        .to_string_bin()
                    );
                    assert_eq!(
                        v[SHR_DEC],
                        (super::build_bigint(v[A_DEC]) >> super::build_bigint(v[SHT_AMT_DEC]))
                            .to_string(),
                    );
                }
            }
        }
    }

    #[test]
    fn bigint_bitand_test() {
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("0b0") & super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("0b1") & super::build_bigint_bin("0b0")).to_string_bin()
        );
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("0b0") & super::build_bigint_bin("0b0")).to_string_bin()
        );
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b1") & super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b1") & super::build_bigint_bin("-0b1")).to_string_bin()
        );
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("-0b1") & super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "-0b1",
            (super::build_bigint_bin("-0b1") & super::build_bigint_bin("-0b1")).to_string_bin()
        );

        if let Ok(lines) = read_lines("./test_inputs.txt") {
            for line in lines {
                if let Ok(testcase) = line {
                    let v: Vec<&str> = testcase.split(',').collect();
                    assert_eq!(
                        v[AND_BIN],
                        (super::build_bigint_bin(v[A_BIN]) & super::build_bigint_bin(v[B_BIN]))
                            .to_string_bin()
                    );
                    assert_eq!(
                        v[AND_DEC],
                        (super::build_bigint(v[A_DEC]) & super::build_bigint(v[B_DEC])).to_string(),
                    );
                }
            }
        }
    }

    #[test]
    fn bigint_bitor_test() {
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b0") | super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b1") | super::build_bigint_bin("0b0")).to_string_bin()
        );
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("0b0") | super::build_bigint_bin("0b0")).to_string_bin()
        );
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b1") | super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "-0b1",
            (super::build_bigint_bin("0b1") | super::build_bigint_bin("-0b1")).to_string_bin()
        );
        assert_eq!(
            "-0b1",
            (super::build_bigint_bin("-0b1") | super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "-0b1",
            (super::build_bigint_bin("-0b1") | super::build_bigint_bin("-0b1")).to_string_bin()
        );

        if let Ok(lines) = read_lines("./test_inputs.txt") {
            for line in lines {
                if let Ok(testcase) = line {
                    let v: Vec<&str> = testcase.split(',').collect();
                    assert_eq!(
                        v[OR_BIN],
                        (super::build_bigint_bin(v[A_BIN]) | super::build_bigint_bin(v[B_BIN]))
                            .to_string_bin()
                    );
                    assert_eq!(
                        v[OR_DEC],
                        (super::build_bigint(v[A_DEC]) | super::build_bigint(v[B_DEC])).to_string(),
                    );
                }
            }
        }
    }

    #[test]
    fn bigint_bitxor_test() {
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b0") ^ super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b1",
            (super::build_bigint_bin("0b1") ^ super::build_bigint_bin("0b0")).to_string_bin()
        );
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("0b0") ^ super::build_bigint_bin("0b0")).to_string_bin()
        );
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("0b1") ^ super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "-0b10",
            (super::build_bigint_bin("0b1") ^ super::build_bigint_bin("-0b1")).to_string_bin()
        );
        assert_eq!(
            "-0b10",
            (super::build_bigint_bin("-0b1") ^ super::build_bigint_bin("0b1")).to_string_bin()
        );
        assert_eq!(
            "0b0",
            (super::build_bigint_bin("-0b1") ^ super::build_bigint_bin("-0b1")).to_string_bin()
        );

        if let Ok(lines) = read_lines("./test_inputs.txt") {
            for line in lines {
                if let Ok(testcase) = line {
                    let v: Vec<&str> = testcase.split(',').collect();
                    assert_eq!(
                        v[XOR_BIN],
                        (super::build_bigint_bin(v[A_BIN]) ^ super::build_bigint_bin(v[B_BIN]))
                            .to_string_bin()
                    );
                    assert_eq!(
                        v[XOR_DEC],
                        (super::build_bigint(v[A_DEC]) ^ super::build_bigint(v[B_DEC])).to_string(),
                    );
                }
            }
        }
    }

    #[test]
    fn bigint_neg_test() {
        assert_eq!("0b0", (-super::build_bigint_bin("0b0")).to_string_bin());

        if let Ok(lines) = read_lines("./test_inputs.txt") {
            for line in lines {
                if let Ok(testcase) = line {
                    let v: Vec<&str> = testcase.split(',').collect();
                    assert_eq!(
                        v[A_NEG_BIN],
                        (-super::build_bigint_bin(v[A_BIN])).to_string_bin()
                    );
                    assert_eq!(v[A_NEG_DEC], (-super::build_bigint(v[A_DEC])).to_string(),);
                }
            }
        }
    }
}
