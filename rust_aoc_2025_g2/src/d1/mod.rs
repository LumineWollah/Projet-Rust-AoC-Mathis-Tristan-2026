#[allow(unused)]
pub fn d1p1_v1(s: &str) -> usize {
    let mut position:usize = 50;
    let mut password = 0;
    // println!("Start position : {}", position);
    for line in s.lines() {
        if line.starts_with('L') {
            let value = line.trim_start_matches('L').parse::<i32>().unwrap();
            let new_position = i32::try_from(position).unwrap();
            let new_position = (new_position - value%100);
            if new_position < 0 {
                position = usize::try_from(100 + new_position).unwrap();
            } else {
                position = usize::try_from(new_position).unwrap();
            }
        } else if line.starts_with('R') {
            let value = line.trim_start_matches('R').parse::<i32>().unwrap();
            let new_position = i32::try_from(position).unwrap();
            let new_position = (new_position + value%100);
            if new_position >= 100 {
                position = usize::try_from(new_position - 100).unwrap();
            } else {
                position = usize::try_from(new_position).unwrap();
            }
        }
        if (position == 0) {
            password += 1;
        }
        // println!("Turn is {}. New position is {}. Password is {}.", position, line, password);
    }
    password
}

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

fn parse_u32_until_eol(bytes: &[u8], mut i: usize) -> (u32, usize) {
    // Parse un entier positif (suite de digits) jusqu'à fin de ligne (ou fin du buffer).
    let mut n: u32 = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_digit() {
            n = n * 10 + u32::from(b - b'0');
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

        //skip espaces si jamais
        while i < bytes.len() && bytes[i] == b' ' {
            i += 1;
        }

        let (value, mut j) = parse_u32_until_eol(bytes, i);
        i = j;

        // saute jusqu'à la fin de ligne
        while i < bytes.len() && bytes[i] != b'\n' && bytes[i] != b'\r' {
            i += 1;
        }

        let step = i32::try_from(value % 100).unwrap();

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
                // ligne invalide -> on ignore
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
        if line.starts_with('L') {
            let mut value = line.trim_start_matches('L').parse::<i32>().unwrap();
            for i in 0..value {
                position -= 1;
                if position == -1 {
                    position = 99;
                }
                if position == 0 {
                    passes += 1;
                }
            }
        } else if line.starts_with('R') {
            let mut value = line.trim_start_matches('R').parse::<i32>().unwrap();
            for i in 0..value {
                position += 1;
                if position == 100 {
                    position = 0;
                }
                if (position == 0) {
                    passes += 1;
                }
            }
        }
        password += passes;
    }
    password
}

fn count_passes_right(pos: i32, k: u32) -> (usize, i32) {
    // anneau taille 100, pos dans [0,99]
    let full = usize::try_from(k / 100).unwrap();
    let r = i32::try_from(k % 100).unwrap();

    // wrap sur le reste si pos + r >= 100
    let wrap = usize::from((pos + r) >= 100);

    let new_pos = (pos + r) % 100;
    (full + wrap, new_pos)
}

fn count_passes_left(pos: i32, k: u32) -> (usize, i32) {
    let full = (k / 100) as usize;
    let r = i32::try_from(k % 100).unwrap();

    // wrap sur le reste si pos - r < 0
    let wrap = usize::from((pos - r) < 0);

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
    #[test]
    fn d1p1_test() {
        let s = include_str!("d1_test.txt");
        let result: usize = super::d1p1(s);
        println!("result: {result}");
        assert_eq!(3, result);
    }

    #[test]
    fn d1p2_test() {
        let s = include_str!("d1_test.txt");
        let result: usize = super::d1p2(s);
        println!("result: {result}");
        assert_eq!(6, result);
    }
}
