fn parse_grid_owned(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

fn parse_grid_borrowed(s: &str) -> Vec<&[u8]> {
    s.lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::as_bytes)
        .collect()
}

fn count_adjacent_rolls_owned(
    grid: &[Vec<u8>],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> usize {
    let y_start = y.saturating_sub(1);
    let y_end = (y + 1).min(height - 1);
    let x_start = x.saturating_sub(1);
    let x_end = (x + 1).min(width - 1);

    let mut adjacent_rolls = 0;

    for (ny, row) in grid
        .iter()
        .enumerate()
        .skip(y_start)
        .take(y_end - y_start + 1)
    {
        for (nx, &cell) in row
            .iter()
            .enumerate()
            .skip(x_start)
            .take(x_end - x_start + 1)
        {
            if nx == x && ny == y {
                continue;
            }

            if cell == b'@' {
                adjacent_rolls += 1;
            }
        }
    }

    adjacent_rolls
}

fn count_adjacent_rolls_borrowed(
    grid: &[&[u8]], 
    x: usize, 
    y: usize, 
    width: usize, 
    height: usize
) -> usize {
    let y_start = y.saturating_sub(1);
    let y_end = (y + 1).min(height - 1);
    let x_start = x.saturating_sub(1);
    let x_end = (x + 1).min(width - 1);

    let mut adjacent_rolls = 0;

    for (ny, row) in grid
        .iter()
        .enumerate()
        .skip(y_start)
        .take(y_end - y_start + 1)
    {
        for (nx, &cell) in row
            .iter()
            .enumerate()
            .skip(x_start)
            .take(x_end - x_start + 1)
        {
            if nx == x && ny == y {
                continue;
            }

            if cell == b'@' {
                adjacent_rolls += 1;
            }
        }
    }

    adjacent_rolls
}

#[allow(unused)]
pub fn d4p1_v1(s: &str) -> usize {
    let grid = parse_grid_owned(s);

    let height = grid.len();
    if height == 0 {
        return 0;
    }
    let width = grid[0].len();

    let mut password = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != b'@' {
                continue;
            }

            let adjacent_rolls = count_adjacent_rolls_owned(&grid, x, y, width, height);

            if adjacent_rolls < 4 {
                password += 1;
            }
        }
    }

    password
}

#[allow(unused)]
pub fn d4p1_v2(s: &str) -> usize {
    // Optimisation: no need to copy lines (using &[u8] instead of Vec<u8>)

    let grid = parse_grid_borrowed(s);

    let height = grid.len();
    if height == 0 {
        return 0;
    }
    let width = grid[0].len();

    let mut password = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != b'@' {
                continue;
            }

            let adjacent_rolls = count_adjacent_rolls_borrowed(&grid, x, y, width, height);

            if adjacent_rolls < 4 {
                password += 1;
            }
        }
    }

    password
}

#[allow(unused)]
pub fn d4p2_v1(s: &str) -> usize {
    let mut grid = parse_grid_owned(s);

    let height = grid.len();
    if height == 0 {
        return 0;
    }
    let width = grid[0].len();

    let mut removed_total = 0;

    loop {
        let mut to_remove = Vec::new();

        for y in 0..height {
            for x in 0..width {
                if grid[y][x] != b'@' {
                    continue;
                }

                let adjacent_rolls = count_adjacent_rolls_owned(&grid, x, y, width, height);

                if adjacent_rolls < 4 {
                    to_remove.push((y, x));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (y, x) in to_remove {
            grid[y][x] = b'.';
            removed_total += 1;
        }
    }

    removed_total
}

#[allow(unused)]
pub fn d4p2_v2(s: &str) -> usize {
    // Optimisation: reuse the removal buffer across iterations to avoid repeated allocations, and batch count removals with to_remove.len()

    let mut grid = parse_grid_owned(s);

    let height = grid.len();
    if height == 0 {
        return 0;
    }
    let width = grid[0].len();

    let mut removed_total = 0;
    let mut to_remove = Vec::with_capacity(width * height);

    loop {
        to_remove.clear();

        for y in 0..height {
            for x in 0..width {
                if grid[y][x] != b'@' {
                    continue;
                }

                let adjacent_rolls = count_adjacent_rolls_owned(&grid, x, y, width, height);

                if adjacent_rolls < 4 {
                    to_remove.push((y, x));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        removed_total += to_remove.len();

        for &(y, x) in &to_remove {
            grid[y][x] = b'.';
        }
    }

    removed_total
}

#[allow(unused)]
pub fn d4p1(s: &str) -> usize {
    d4p1_v2(s)
}

#[allow(unused)]
pub fn d4p2(s: &str) -> usize {
    d4p2_v2(s)
}

#[cfg(test)]
mod tests {
    use crate::d4::{d4p1, d4p2};

    #[test]
    fn d4p1_test() {
        let s = include_str!("d4_test.txt");
        let result = d4p1(s);
        println!("result: {result}");
        assert_eq!(13, result);
    }

    #[test]
    fn d4p2_test() {
        let s = include_str!("d4_test.txt");
        let result = d4p2(s);
        println!("result: {result}");
        assert_eq!(43, result);
    }
}