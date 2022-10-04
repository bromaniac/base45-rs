use std::collections::HashMap;

pub fn new() -> HashMap<usize, char> {
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

pub fn b45encode(v: Vec<u8>) -> String {
    let dict = new();
    let mut res = String::new();

    for s in v.chunks(2) {
        if s.len() > 1 {
            let l: usize = (s[0] as usize * 256 + s[1] as usize).into();

            let (n, c) = divmod(l);
            let (o, d) = divmod(n);
            let (_, e) = divmod(o);

            res.push(dict[&c]);
            res.push(dict[&d]);
            res.push(dict[&e]);
        } else {
            let (d, c) = divmod(s[0].into());
            res.push(dict[&c]);
            res.push(dict[&d]);
        }
    }
    res
}

pub fn b45decode() {
    unimplemented!();
}

/// Calculate result and remainder
fn divmod(u: usize) -> (usize, usize) {
    let l = u % 45;
    let m = (u - l) / 45;
    (m, l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divmod() {
        assert_eq!(divmod(26725), (593, 40));
        assert_eq!(divmod(593), (13, 8));
        assert_eq!(divmod(13), (0, 13));
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
}
