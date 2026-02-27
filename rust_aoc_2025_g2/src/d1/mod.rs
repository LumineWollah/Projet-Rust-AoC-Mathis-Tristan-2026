use std::time::Duration;

#[allow(unused)]
pub fn d1p1_v1(s: &str) -> usize {
    let mut position:usize = 50;
    let mut password = 0;
    // println!("Start position : {}", position);
    for line in s.lines() {
        if line.starts_with("L") {
            let value = line.trim_start_matches("L").parse::<i32>().unwrap();
            let new_position = position as i32;
            let new_position = (new_position - value%100);
            if new_position < 0 {
                position = (100 + new_position) as usize;
            } else {
                position = new_position as usize;
            }
        } else if line.starts_with("R") {
            let value = line.trim_start_matches("R").parse::<i32>().unwrap();
            let new_position = position as i32;
            let new_position = (new_position + value%100);
            if new_position >= 100 {
                position = (new_position - 100) as usize;
            } else {
                position = new_position as usize;
            }
        }
        if (position == 0) {
            password = password + 1;
        }
        // println!("Turn is {}. New position is {}. Password is {}.", position, line, password);
    }
    password
}

#[inline(always)]
fn skip_newlines(bytes: &[u8], mut i: usize) -> usize {
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\n' || b == b'\r' {
            i += 1;
        } else {
            break;
        }
    }
    i
}

#[inline(always)]
fn parse_u32_until_eol(bytes: &[u8], mut i: usize) -> (u32, usize) {
    // Parse un entier positif (suite de digits) jusqu'à fin de ligne (ou fin du buffer).
    let mut n: u32 = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if (b'0'..=b'9').contains(&b) {
            n = n * 10 + (b - b'0') as u32;
            i += 1;
        } else {
            break;
        }
    }
    (n, i)
}

#[allow(unused)]
pub fn d1p1_v2(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut i: usize = 0;

    let mut position: i32 = 50;
    let mut password: usize = 0;

    while i < bytes.len() {
        i = skip_newlines(bytes, i);
        if i >= bytes.len() {
            break;
        }

        let op = bytes[i];
        i += 1;

        // (optionnel) skip espaces si jamais
        while i < bytes.len() && bytes[i] == b' ' {
            i += 1;
        }

        let (value, mut j) = parse_u32_until_eol(bytes, i);
        i = j;

        // saute jusqu'à la fin de ligne (au cas où il reste des espaces/tabs)
        while i < bytes.len() && bytes[i] != b'\n' && bytes[i] != b'\r' {
            i += 1;
        }

        let step = (value % 100) as i32;

        match op {
            b'L' => {
                // position = (position - step) mod 100
                let mut new_pos = position - step;
                if new_pos < 0 {
                    new_pos += 100;
                }
                position = new_pos;
            }
            b'R' => {
                // position = (position + step) mod 100
                let mut new_pos = position + step;
                if new_pos >= 100 {
                    new_pos -= 100;
                }
                position = new_pos;
            }
            _ => {
                // ligne invalide -> on ignore (ou tu peux panic si tu veux)
            }
        }
        if position == 0 {
            password += 1;
        }
    }
    password
}

#[allow(unused)]
pub fn d1p2_v1(s: &str) -> usize {
    let mut position:i32 = 50;
    let mut password:usize = 0;
    for line in s.lines() {
        let mut passes = 0;
        if line.starts_with("L") {
            let mut value = line.trim_start_matches("L").parse::<i32>().unwrap();
            for i in 0..value {
                position = position - 1;
                if position == -1 {
                    position = 99;
                }
                if position == 0 {
                    passes = passes + 1;
                }
            }
        } else if line.starts_with("R") {
            let mut value = line.trim_start_matches("R").parse::<i32>().unwrap();
            for i in 0..value {
                position = position + 1;
                if position == 100 {
                    position = 0;
                }
                if (position == 0) {
                    passes = passes + 1;
                }
            }
        }
        password = password + passes as usize;
    }
    password
}

#[inline(always)]
fn count_passes_right(pos: i32, k: u32) -> (usize, i32) {
    // anneau taille 100, pos dans [0,99]
    let full = (k / 100) as usize;
    let r = (k % 100) as i32;

    // wrap sur le reste si pos + r >= 100
    let wrap = ((pos + r) >= 100) as usize;

    let new_pos = (pos + r) % 100;
    (full + wrap, new_pos)
}

#[inline(always)]
fn count_passes_left(pos: i32, k: u32) -> (usize, i32) {
    let full = (k / 100) as usize;
    let r = (k % 100) as i32;

    // wrap sur le reste si pos - r < 0
    let wrap = ((pos - r) < 0) as usize;

    let mut new_pos = pos - r;
    new_pos %= 100;
    if new_pos < 0 {
        new_pos += 100;
    }
    (full + wrap, new_pos)
}

#[allow(unused)]
pub fn d1p2_v2(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut i: usize = 0;

    let mut position: i32 = 50;
    let mut password: usize = 0;

    while i < bytes.len() {
        i = skip_newlines(bytes, i);
        if i >= bytes.len() {
            break;
        }

        let op = bytes[i];
        i += 1;

        // (optionnel) skip espaces
        while i < bytes.len() && bytes[i] == b' ' {
            i += 1;
        }

        let (value, mut j) = parse_u32_until_eol(bytes, i);
        i = j;

        // saute jusqu'à fin de ligne (si espaces/tabs restants)
        while i < bytes.len() && bytes[i] != b'\n' && bytes[i] != b'\r' {
            i += 1;
        }

        match op {
            b'R' => {
                let (passes, new_pos) = count_passes_right(position, value);
                password += passes;
                position = new_pos;
            }
            b'L' => {
                let (passes, new_pos) = count_passes_left(position, value);
                password += passes;
                position = new_pos;
            }
            _ => {
                // ignore ligne invalide (ou panic)
            }
        }
    }

    password
}

#[allow(unused)]
pub fn d1p1(s: &str) -> usize {
    d1p1_v1(s)
}
#[allow(unused)]
pub fn d1p2(s: &str) -> usize {
    d1p2_v1(s)
}

#[cfg(test)]
mod tests {
    use crate::d1::{d1p1, d1p2};

    #[test]
    fn d1p1_test() {
        let s = include_str!("d1_test.txt");
        let result: usize = d1p1(s);
        println!("result: {}", result);
        assert_eq!(3, result);
    }

    #[test]
    fn d1p2_test() {
        let s = include_str!("d1_test.txt");
        let result: usize = d1p2(s);
        println!("result: {}", result);
        assert_eq!(6, result);
    }
}
