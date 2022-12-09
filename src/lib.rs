use std::collections::HashMap;
use thiserror::Error;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Error, Debug)]
pub enum Base45DecodeError {
    #[error("Invalid base45 string")]
    ValueError,
}

fn new_encode_lut() -> HashMap<usize, char> {
    let base45_charset = vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ' ', '$', '%', '*', '+', '-', '.', '/', ':',
    ];

    let mut base45map = HashMap::new();
    for (i, x) in base45_charset.iter().enumerate() {
        base45map.insert(i, x.clone());
    }
    base45map
}

fn new_decode_lut() -> HashMap<char, usize> {
    let base45_charset = vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ' ', '$', '%', '*', '+', '-', '.', '/', ':',
    ];

    let mut base45map = HashMap::new();
    for (i, x) in base45_charset.iter().enumerate() {
        base45map.insert(x.clone(), i);
    }
    base45map
}

pub fn b45encode(v: Vec<u8>) -> String {
    let dict = new_encode_lut();
    let mut res = String::new();

    for s in v.chunks(2) {
        if s.len() > 1 {
            let l: usize = (s[0] as usize * 256 + s[1] as usize).into();

            let (n, c) = divmod(l, 45);
            let (o, d) = divmod(n, 45);
            let (_, e) = divmod(o, 45);

            res.push(dict[&c]);
            res.push(dict[&d]);
            res.push(dict[&e]);
        } else {
            let (d, c) = divmod(s[0].into(), 45);
            res.push(dict[&c]);
            res.push(dict[&d]);
        }
    }
    res
}

pub fn b45decode(s: &str) -> MyResult<Vec<usize>> {
    let dict = new_decode_lut();
    let mut buf = Vec::new();

    for c in s.as_bytes() {
        let k = *c as char;
        buf.push(dict[&k]);
    }

    let buflen = buf.len();
    if buflen % 3 == 1 {
        return Err(Box::new(Base45DecodeError::ValueError));
    }

    let mut res = Vec::new();
    for i in (0..buflen).step_by(3) {
        if buflen - i >= 3 {
            let x = buf[i] + buf[i + 1] * 45 + buf[i + 2] * 45 * 45;
            if x > 0xFFFF {
                return Err(Box::new(Base45DecodeError::ValueError));
            }

            let (q, r) = divmod(x, 256);
            res.push(q);
            res.push(r);
        } else {
            let x = buf[i] + buf[i + 1] * 45;
            if x > 0xFFFF {
                return Err(Box::new(Base45DecodeError::ValueError));
            }

            res.push(x);
        }
    }

    Ok(res)
}

/// Calculate result and remainder
fn divmod(a: usize, n: usize) -> (usize, usize) {
    let l = a % n;
    let m = (a - l) / n;
    (m, l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divmod() {
        assert_eq!(divmod(26725, 45), (593, 40));
        assert_eq!(divmod(593, 45), (13, 8));
        assert_eq!(divmod(13, 45), (0, 13));
    }

    #[test]
    fn test_b45encode_ab() {
        let result = b45encode(vec![65, 66]);
        assert_eq!(result, "BB8");
    }

    #[test]
    fn test_b45encode_hello() {
        let result = b45encode(vec![72, 101, 108, 108, 111, 33, 33]);
        assert_eq!(result, "%69 VD92EX0");
    }

    #[test]
    fn test_b45encode_base_dash_45() {
        let result = b45encode(vec![98, 97, 115, 101, 45, 52, 53]);
        assert_eq!(result, "UJCLQE7W581");
    }

    #[test]
    fn test_b45decode() {
        let result = b45decode("QED8WEX0").unwrap();
        assert_eq!(result, [105, 101, 116, 102, 33]); // ietf!
    }
}
