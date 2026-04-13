fn parse_lines_str(s: &str) -> Vec<&str> {
    s.lines().collect()
}

fn parse_lines_bytes(s: &str) -> Vec<&[u8]> {
    s.lines()
        .filter(|line| !line.is_empty())
        .map(str::as_bytes)
        .collect()
}

fn max_width_str(lines: &[&str]) -> usize {
    lines.iter().map(|line| line.len()).max().unwrap_or(0)
}

fn max_width_bytes(lines: &[&[u8]]) -> usize {
    lines.iter().map(|line| line.len()).max().unwrap_or(0)
}

fn padded_grid(lines: &[&str], width: usize) -> Vec<Vec<u8>> {
    lines.iter()
        .map(|line| {
            let mut row = line.as_bytes().to_vec();
            row.resize(width, b' ');
            row
        })
        .collect()
}

fn is_blank_column_padded(grid: &[Vec<u8>], x: usize) -> bool {
    grid.iter().all(|row| row[x] == b' ')
}

fn is_blank_column_bytes(lines: &[&[u8]], x: usize) -> bool {
    lines.iter().all(|row| x >= row.len() || row[x] == b' ')
}

fn parse_ascii_usize(bytes: &[u8]) -> usize {
    bytes.iter().fold(0_usize, |acc, &c| {
        acc * 10 + usize::from(c.saturating_sub(b'0'))
    })
}

fn eval_numbers(numbers: Vec<usize>, op: u8) -> usize {
    match op {
        b'+' => numbers.into_iter().sum(),
        b'*' => numbers.into_iter().product(),
        _ => 0,
    }
}

#[allow(unused)]
pub fn d6p1_v1(s: &str) -> usize {
    let lines = parse_lines_str(s);
    if lines.is_empty() {
        return 0;
    }

    let width = max_width_str(&lines);
    if width == 0 {
        return 0;
    }

    let padded = padded_grid(&lines, width);
    let mut total = 0_usize;
    let mut x = 0_usize;

    while x < width {
        if is_blank_column_padded(&padded, x) {
            x += 1;
            continue;
        }

        let start = x;
        while x < width && !is_blank_column_padded(&padded, x) {
            x += 1;
        }
        let end = x;

        let mut numbers = Vec::new();
        let mut op = b' ';

        for row in &padded {
            let slice = &row[start..end];
            let Ok(trimmed) = std::str::from_utf8(slice) else {
                continue;
            };
            let trimmed = trimmed.trim();

            if trimmed.is_empty() {
                continue;
            }

            if trimmed == "*" || trimmed == "+" {
                op = trimmed.as_bytes()[0];
            } else if let Ok(value) = trimmed.parse::<usize>() {
                numbers.push(value);
            }
        }

        total += eval_numbers(numbers, op);
    }

    total
}

#[allow(unused)]
pub fn d6p1_v2(s: &str) -> usize {
    // Optimisation: avoid padding and string conversions, work directly with byte slices

    let lines = parse_lines_bytes(s);
    if lines.is_empty() {
        return 0;
    }

    let width = max_width_bytes(&lines);
    if width == 0 {
        return 0;
    }

    let mut total = 0_usize;
    let mut x = 0_usize;

    while x < width {
        if is_blank_column_bytes(&lines, x) {
            x += 1;
            continue;
        }

        let start = x;
        while x < width && !is_blank_column_bytes(&lines, x) {
            x += 1;
        }
        let end = x;

        let mut numbers = Vec::new();
        let mut op = b' ';

        for row in &lines {
            let slice_end = end.min(row.len());
            if start >= slice_end {
                continue;
            }

            let slice = &row[start..slice_end];

            let left = slice.iter().position(|&c| c != b' ');
            let right = slice.iter().rposition(|&c| c != b' ');

            let (Some(left_idx), Some(right_idx)) = (left, right) else {
                continue;
            };

            let trimmed = &slice[left_idx..=right_idx];

            if trimmed.len() == 1 && (trimmed[0] == b'+' || trimmed[0] == b'*') {
                op = trimmed[0];
            } else {
                numbers.push(parse_ascii_usize(trimmed));
            }
        }

        total += eval_numbers(numbers, op);
    }

    total
}

#[allow(unused)]
pub fn d6p2_v1(s: &str) -> usize {
    let lines = parse_lines_str(s);
    if lines.is_empty() {
        return 0;
    }

    let width = max_width_str(&lines);
    if width == 0 {
        return 0;
    }

    let padded = padded_grid(&lines, width);
    let mut total = 0_usize;
    let mut x = 0_usize;

    while x < width {
        if is_blank_column_padded(&padded, x) {
            x += 1;
            continue;
        }

        let start = x;
        while x < width && !is_blank_column_padded(&padded, x) {
            x += 1;
        }
        let end = x;

        let mut numbers = Vec::new();
        let mut op = b' ';

        for row in &padded {
            for &c in &row[start..end] {
                if c == b'+' || c == b'*' {
                    op = c;
                }
            }
        }

        for xx in (start..end).rev() {
            let digits: Vec<u8> = padded
                .iter()
                .filter_map(|row| {
                    let c = row[xx];
                    c.is_ascii_digit().then_some(c)
                })
                .collect();

            if !digits.is_empty() {
                numbers.push(parse_ascii_usize(&digits));
            }
        }

        total += eval_numbers(numbers, op);
    }

    total
}

#[allow(unused)]
pub fn d6p2_v2(s: &str) -> usize {
    // Optimisation: avoid padding and string conversions, work directly with byte slices

    let lines = parse_lines_bytes(s);
    if lines.is_empty() {
        return 0;
    }

    let width = max_width_bytes(&lines);
    if width == 0 {
        return 0;
    }

    let mut total = 0_usize;
    let mut x = 0_usize;

    while x < width {
        if is_blank_column_bytes(&lines, x) {
            x += 1;
            continue;
        }

        let start = x;
        while x < width && !is_blank_column_bytes(&lines, x) {
            x += 1;
        }
        let end = x;

        let mut op = b' ';
        let mut numbers = Vec::new();

        for row in &lines {
            let slice_end = end.min(row.len());
            if start >= slice_end {
                continue;
            }

            for &c in &row[start..slice_end] {
                if c == b'+' || c == b'*' {
                    op = c;
                    break;
                }
            }

            if op != b' ' {
                break;
            }
        }

        for xx in (start..end).rev() {
            let mut value = 0_usize;
            let mut has_digit = false;

            for row in &lines {
                if let Some(&c) = row.get(xx)
                    && c.is_ascii_digit()
                {
                    value = value * 10 + usize::from(c - b'0');
                    has_digit = true;
                }
            }

            if has_digit {
                numbers.push(value);
            }
        }

        total += eval_numbers(numbers, op);
    }

    total
}

#[allow(unused)]
pub fn d6p1(s: &str) -> usize {
    d6p1_v2(s)
}

#[allow(unused)]
pub fn d6p2(s: &str) -> usize {
    d6p2_v2(s)
}

#[cfg(test)]
mod tests {
    use crate::d6::{d6p1, d6p2};

    #[test]
    fn d6p1_test() {
        let s = include_str!("d6_test.txt");
        let result = d6p1(s);
        println!("result: {result}");
        assert_eq!(4_277_556, result);
    }

    #[test]
    fn d6p2_test() {
        let s = include_str!("d6_test.txt");
        let result = d6p2(s);
        println!("result: {result}");
        assert_eq!(3_263_827, result);
    }
}