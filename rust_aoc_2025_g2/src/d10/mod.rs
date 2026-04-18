use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn extract_groups<'a>(s: &'a str, open: u8, close: u8) -> Vec<&'a str> {
    let bytes = s.as_bytes();
    let mut groups = Vec::new();
    let mut start = None;

    for (idx, &byte) in bytes.iter().enumerate() {
        if byte == open {
            start = Some(idx + 1);
        } else if byte == close {
            if let Some(group_start) = start {
                groups.push(&s[group_start..idx]);
            }
            start = None;
        }
    }

    groups
}

fn parse_indices(group: &str) -> Vec<usize> {
    if group.trim().is_empty() {
        return Vec::new();
    }

    group
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .filter_map(|part| part.parse::<usize>().ok())
        .collect()
}

fn parse_machine(line: &str) -> Option<(Vec<bool>, Vec<Vec<usize>>)> {
    let diagrams = extract_groups(line, b'[', b']');
    let target_group = diagrams.first()?;

    let target = target_group
        .bytes()
        .filter_map(|byte| match byte {
            b'.' => Some(false),
            b'#' => Some(true),
            _ => None,
        })
        .collect::<Vec<bool>>();

    let buttons = extract_groups(line, b'(', b')')
        .into_iter()
        .map(parse_indices)
        .collect::<Vec<Vec<usize>>>();

    Some((target, buttons))
}

fn is_target_reached(current: &[bool], target: &[bool]) -> bool {
    current == target
}

fn toggle_button(current: &mut [bool], button: &[usize]) {
    for &idx in button {
        if let Some(light) = current.get_mut(idx) {
            *light = !*light;
        }
    }
}

fn solve_machine_v1(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    fn dfs(
        button_idx: usize,
        current: &mut [bool],
        target: &[bool],
        buttons: &[Vec<usize>],
        presses_so_far: usize,
        best: &mut usize,
    ) {
        if presses_so_far >= *best {
            return;
        }

        if button_idx == buttons.len() {
            if is_target_reached(current, target) {
                *best = presses_so_far;
            }
            return;
        }

        dfs(
            button_idx + 1,
            current,
            target,
            buttons,
            presses_so_far,
            best,
        );

        toggle_button(current, &buttons[button_idx]);
        dfs(
            button_idx + 1,
            current,
            target,
            buttons,
            presses_so_far + 1,
            best,
        );
        toggle_button(current, &buttons[button_idx]);
    }

    let mut current = vec![false; target.len()];
    let mut best = usize::MAX;

    dfs(0, &mut current, target, buttons, 0, &mut best);

    if best == usize::MAX { 0 } else { best }
}

#[allow(unused)]
pub fn d10p1_v1(s: &str) -> usize {
    s.lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(parse_machine)
        .map(|(target, buttons)| solve_machine_v1(&target, &buttons))
        .sum()
}

fn solve_machine_v2(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    let light_count = target.len();
    let button_count = buttons.len();

    if light_count == 0 || button_count == 0 {
        return 0;
    }

    let mut matrix = vec![vec![false; button_count + 1]; light_count];

    for (row_idx, &target_value) in target.iter().enumerate() {
        matrix[row_idx][button_count] = target_value;
    }

    for (col_idx, button) in buttons.iter().enumerate() {
        for &light_idx in button {
            if let Some(row) = matrix.get_mut(light_idx) {
                row[col_idx] = !row[col_idx];
            }
        }
    }

    let mut pivot_cols = Vec::new();
    let mut row = 0_usize;

    for col in 0..button_count {
        let pivot_row = (row..light_count).find(|&candidate_row| matrix[candidate_row][col]);

        let Some(pivot_row) = pivot_row else {
            continue;
        };

        matrix.swap(row, pivot_row);

        for other_row in 0..light_count {
            if other_row != row && matrix[other_row][col] {
                for bit_idx in col..=button_count {
                    matrix[other_row][bit_idx] ^= matrix[row][bit_idx];
                }
            }
        }

        pivot_cols.push(col);
        row += 1;

        if row == light_count {
            break;
        }
    }

    for current_row in matrix.iter().skip(row) {
        if current_row[..button_count].iter().all(|&value| !value) && current_row[button_count] {
            return 0;
        }
    }

    let mut is_pivot_col = vec![false; button_count];
    for &pivot_col in &pivot_cols {
        is_pivot_col[pivot_col] = true;
    }

    let free_cols = (0..button_count)
        .filter(|&col| !is_pivot_col[col])
        .collect::<Vec<usize>>();

    fn count_presses_for_assignment(
        matrix: &[Vec<bool>],
        pivot_cols: &[usize],
        free_cols: &[usize],
        free_mask: usize,
        button_count: usize,
    ) -> usize {
        let mut solution = vec![false; button_count];

        for (free_idx, &col) in free_cols.iter().enumerate() {
            if ((free_mask >> free_idx) & 1) == 1 {
                solution[col] = true;
            }
        }

        for (row_idx, &pivot_col) in pivot_cols.iter().enumerate().rev() {
            let mut value = matrix[row_idx][button_count];

            for (col_idx, &cell) in matrix[row_idx][..button_count].iter().enumerate() {
                if col_idx != pivot_col && cell && solution[col_idx] {
                    value = !value;
                }
            }

            solution[pivot_col] = value;
        }

        solution.into_iter().filter(|pressed| *pressed).count()
    }

    let too_many_free_cols = match u32::try_from(free_cols.len()) {
        Ok(free_col_count) => free_col_count >= usize::BITS,
        Err(_) => true,
    };

    if too_many_free_cols {
        return solve_machine_v1(target, buttons);
    }

    let combinations = 1_usize << free_cols.len();
    let mut best = usize::MAX;

    for free_mask in 0..combinations {
        let presses =
            count_presses_for_assignment(&matrix, &pivot_cols, &free_cols, free_mask, button_count);
        best = best.min(presses);
    }

    if best == usize::MAX { 0 } else { best }
}

#[allow(unused)]
pub fn d10p1_v2(s: &str) -> usize {
    // Optimisation: instead of brute-forcing every button combination, solve each machine with Gaussian elimination over GF(2) and only brute-force the free variables

    s.lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(parse_machine)
        .map(|(target, buttons)| solve_machine_v2(&target, &buttons))
        .sum()
}

pub fn d10p2_v1(s: &str) -> usize {
    use std::cmp::Reverse;

    #[derive(Clone, Debug)]
    struct Frame {
        next_candidate_idx: usize,
        candidates: Vec<usize>,
    }

    fn extract_groups<'a>(s: &'a str, open: u8, close: u8) -> Vec<&'a str> {
        let bytes = s.as_bytes();
        let mut groups = Vec::new();
        let mut start = None;

        for (idx, &byte) in bytes.iter().enumerate() {
            if byte == open {
                start = Some(idx + 1);
            } else if byte == close {
                if let Some(group_start) = start {
                    groups.push(&s[group_start..idx]);
                }
                start = None;
            }
        }

        groups
    }

    fn parse_number_list(group: &str) -> Vec<usize> {
        if group.trim().is_empty() {
            return Vec::new();
        }

        group
            .split(',')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .map(|part| {
                part.parse::<usize>()
                    .expect("button index must be a valid non-negative integer")
            })
            .collect()
    }

    fn parse_target(group: &str) -> Vec<u16> {
        if group.trim().is_empty() {
            return Vec::new();
        }

        group
            .split(',')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .map(|part| {
                part.parse::<u16>()
                    .expect("target joltage must be a valid non-negative integer")
            })
            .collect()
    }

    fn parse_machine(line: &str) -> Option<(Vec<Vec<usize>>, Vec<u16>)> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return None;
        }

        let target_group = extract_groups(trimmed, b'{', b'}').into_iter().next()?;
        let targets = parse_target(target_group);

        let prefix = trimmed.split('{').next()?;
        let mut buttons = extract_groups(prefix, b'(', b')')
            .into_iter()
            .map(parse_number_list)
            .filter(|button| !button.is_empty())
            .map(|mut button| {
                button.sort_unstable();
                button.dedup();
                button
            })
            .collect::<Vec<_>>();

        buttons.sort_unstable();
        buttons.dedup();

        Some((buttons, targets))
    }

    fn is_zero_state(state: &[u16]) -> bool {
        state.iter().all(|&value| value == 0)
    }

    fn lower_bound(state: &[u16], max_button_coverage: usize) -> usize {
        let max_needed = state.iter().copied().map(usize::from).max().unwrap_or(0);
        let total_needed = state.iter().copied().map(usize::from).sum::<usize>();

        let coverage_bound = if max_button_coverage == 0 {
            0
        } else {
            total_needed.div_ceil(max_button_coverage)
        };

        max_needed.max(coverage_bound)
    }

    fn can_apply_button(state: &[u16], button: &[usize]) -> bool {
        button.iter().all(|&index| state[index] > 0)
    }

    fn apply_button(state: &mut [u16], button: &[usize]) {
        for &index in button {
            state[index] -= 1;
        }
    }

    fn undo_button(state: &mut [u16], button: &[usize]) {
        for &index in button {
            state[index] += 1;
        }
    }

    fn button_score(state: &[u16], button: &[usize]) -> usize {
        button.iter().map(|&index| usize::from(state[index])).sum()
    }

    fn build_candidates(state: &[u16], buttons: &[Vec<usize>]) -> Vec<usize> {
        let mut candidates = buttons
            .iter()
            .enumerate()
            .filter(|(_, button)| can_apply_button(state, button))
            .map(|(idx, button)| {
                (
                    Reverse(button_score(state, button)),
                    Reverse(button.len()),
                    idx,
                )
            })
            .collect::<Vec<_>>();

        candidates.sort_unstable();
        candidates.into_iter().map(|(_, _, idx)| idx).collect()
    }

    fn ida_search_iterative(
        initial_state: &[u16],
        buttons: &[Vec<usize>],
        max_button_coverage: usize,
        bound: usize,
    ) -> Result<usize, usize> {
        let root_heuristic = lower_bound(initial_state, max_button_coverage);
        if root_heuristic > bound {
            return Err(root_heuristic);
        }

        if root_heuristic == 0 {
            return Ok(0);
        }

        let mut state = initial_state.to_vec();
        let mut stack = Vec::<Frame>::new();
        let mut applied_path = Vec::<usize>::new();
        let mut next_bound = usize::MAX;

        stack.push(Frame {
            next_candidate_idx: 0,
            candidates: build_candidates(&state, buttons),
        });

        loop {
            let depth = applied_path.len();

            if is_zero_state(&state) {
                return Ok(depth);
            }

            if let Some(frame) = stack.last_mut() {
                if frame.next_candidate_idx >= frame.candidates.len() {
                    stack.pop();

                    if let Some(last_button_idx) = applied_path.pop() {
                        undo_button(&mut state, &buttons[last_button_idx]);
                        continue;
                    }

                    break;
                }

                let button_idx = frame.candidates[frame.next_candidate_idx];
                frame.next_candidate_idx += 1;

                apply_button(&mut state, &buttons[button_idx]);

                let next_depth = depth + 1;
                let heuristic = lower_bound(&state, max_button_coverage);
                let estimate = next_depth + heuristic;

                if estimate > bound {
                    next_bound = next_bound.min(estimate);
                    undo_button(&mut state, &buttons[button_idx]);
                    continue;
                }

                if heuristic == 0 {
                    return Ok(next_depth);
                }

                applied_path.push(button_idx);

                stack.push(Frame {
                    next_candidate_idx: 0,
                    candidates: build_candidates(&state, buttons),
                });
            } else {
                break;
            }
        }

        Err(next_bound)
    }

    fn min_presses(buttons: &[Vec<usize>], targets: &[u16]) -> Option<usize> {
        if is_zero_state(targets) {
            return Some(0);
        }

        if buttons.is_empty() {
            return None;
        }

        let counter_count = targets.len();

        let mut usable_buttons = buttons
            .iter()
            .filter(|button| button.iter().all(|&index| index < counter_count))
            .filter(|button| !button.is_empty())
            .cloned()
            .collect::<Vec<_>>();

        if usable_buttons.is_empty() {
            return None;
        }

        let mut covered = vec![false; counter_count];
        for button in &usable_buttons {
            for &index in button {
                covered[index] = true;
            }
        }

        if targets
            .iter()
            .enumerate()
            .any(|(index, &target)| target > 0 && !covered[index])
        {
            return None;
        }

        usable_buttons.sort_by_key(|button| Reverse(button.len()));

        let max_button_coverage = usable_buttons.iter().map(Vec::len).max().unwrap_or(0);
        let mut bound = lower_bound(targets, max_button_coverage);

        loop {
            match ida_search_iterative(targets, &usable_buttons, max_button_coverage, bound) {
                Ok(answer) => return Some(answer),
                Err(next_bound) => {
                    if next_bound == usize::MAX {
                        return None;
                    }
                    bound = next_bound;
                }
            }
        }
    }

    s.lines()
        .filter_map(parse_machine)
        .map(|(buttons, targets)| min_presses(&buttons, &targets).expect("machine is unsatisfiable"))
        .sum()
}

#[allow(unused)]
pub fn d10p1(s: &str) -> usize {
    d10p1_v2(s)
}

#[allow(unused)]
pub fn d10p2(s: &str) -> usize {
    d10p2_v1(s)
}

#[cfg(test)]
mod tests {
    use crate::d10::{d10p1,d10p2};

    #[test]
    fn d10p1_test() {
        let s = include_str!("d10_test.txt");
        let result = d10p1(s);
        println!("result: {result}");
        assert_eq!(7, result);
    }

    #[test]
    fn d10p2_test() {
        let s = include_str!("d10_test.txt");
        let result = d10p2(s);
        println!("result: {result}");
        assert_eq!(33, result);
    }
}