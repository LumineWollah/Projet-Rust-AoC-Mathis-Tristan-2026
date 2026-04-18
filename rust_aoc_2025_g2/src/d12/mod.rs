use std::cmp::Reverse;

use std::collections::HashSet;

pub fn d12p1_v1(s: &str) -> usize {
    #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Orientation {
        cells: Vec<(usize, usize)>,
        width: usize,
        height: usize,
    }

    #[derive(Clone, Debug)]
    struct Placement {
        bits: Vec<u64>,
    }

    #[derive(Clone, Debug)]
    struct ShapeData {
        area: usize,
        placements: Vec<Placement>,
    }

    #[derive(Clone, Debug)]
    struct Region {
        width: usize,
        height: usize,
        counts: Vec<usize>,
    }

    fn is_shape_row(line: &str) -> bool {
        !line.is_empty() && line.bytes().all(|byte| byte == b'#' || byte == b'.')
    }

    fn parse_shape_header(line: &str) -> Option<usize> {
        let trimmed = line.trim();
        let (left, right) = trimmed.split_once(':')?;
        if !right.trim().is_empty() {
            return None;
        }
        left.trim().parse::<usize>().ok()
    }

    fn parse_region(line: &str) -> Option<Region> {
        let trimmed = line.trim();
        let (dims, counts_part) = trimmed.split_once(':')?;
        let (width_str, height_str) = dims.trim().split_once('x')?;
        let width = width_str.trim().parse::<usize>().ok()?;
        let height = height_str.trim().parse::<usize>().ok()?;

        let counts = counts_part
            .split_whitespace()
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        Some(Region {
            width,
            height,
            counts,
        })
    }

    fn normalize(points: &[(isize, isize)]) -> Orientation {
        let min_x = points.iter().map(|&(x, _)| x).min().unwrap_or(0);
        let min_y = points.iter().map(|&(_, y)| y).min().unwrap_or(0);

        let mut cells = points
            .iter()
            .map(|&(x, y)| {
                let nx = usize::try_from(x - min_x)
                    .expect("normalized x coordinate must be non-negative");
                let ny = usize::try_from(y - min_y)
                    .expect("normalized y coordinate must be non-negative");
                (nx, ny)
            })
            .collect::<Vec<_>>();

        cells.sort_unstable();

        let width = cells
            .iter()
            .map(|&(x, _)| x)
            .max()
            .map_or(0, |value| value + 1);

        let height = cells
            .iter()
            .map(|&(_, y)| y)
            .max()
            .map_or(0, |value| value + 1);

        Orientation {
            cells,
            width,
            height,
        }
    }

    fn make_orientations(shape_rows: &[String]) -> Vec<Orientation> {
        let mut base_points = Vec::new();

        for (y, row) in shape_rows.iter().enumerate() {
            for (x, byte) in row.bytes().enumerate() {
                if byte == b'#' {
                    base_points.push((
                        isize::try_from(x).expect("x must fit into isize"),
                        isize::try_from(y).expect("y must fit into isize"),
                    ));
                }
            }
        }

        let transforms: [fn(isize, isize) -> (isize, isize); 8] = [
            |x, y| (x, y),
            |x, y| (x, -y),
            |x, y| (-x, y),
            |x, y| (-x, -y),
            |x, y| (y, x),
            |x, y| (y, -x),
            |x, y| (-y, x),
            |x, y| (-y, -x),
        ];

        let mut orientations = Vec::new();

        for transform in transforms {
            let transformed = base_points
                .iter()
                .map(|&(x, y)| transform(x, y))
                .collect::<Vec<_>>();

            let normalized = normalize(&transformed);

            if !orientations.contains(&normalized) {
                orientations.push(normalized);
            }
        }

        orientations
    }

    fn parse_input(input: &str) -> (Vec<Vec<Orientation>>, Vec<usize>, Vec<Region>) {
        let mut raw_shapes: Vec<Option<Vec<String>>> = Vec::new();
        let mut regions = Vec::new();
        let mut lines = input.lines().peekable();

        while let Some(line) = lines.next() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            if let Some(index) = parse_shape_header(trimmed) {
                let mut rows = Vec::new();

                while let Some(next_line) = lines.peek() {
                    let next_trimmed = next_line.trim();
                    if !is_shape_row(next_trimmed) {
                        break;
                    }
                    rows.push(next_trimmed.to_owned());
                    let _ = lines.next();
                }

                if raw_shapes.len() <= index {
                    raw_shapes.resize_with(index + 1, || None);
                }
                raw_shapes[index] = Some(rows);
            } else if let Some(region) = parse_region(trimmed) {
                regions.push(region);
            }
        }

        let mut shapes = Vec::new();
        let mut shape_areas = Vec::new();

        for maybe_rows in raw_shapes {
            let rows = maybe_rows.expect("shape indices must be contiguous");
            let orientations = make_orientations(&rows);
            let area = orientations
                .first()
                .map(|orientation| orientation.cells.len())
                .unwrap_or(0);

            shapes.push(orientations);
            shape_areas.push(area);
        }

        (shapes, shape_areas, regions)
    }

    fn build_placements(
        orientations: &[Orientation],
        region_width: usize,
        region_height: usize,
    ) -> Vec<Placement> {
        let cell_count = region_width * region_height;
        let word_count = cell_count.div_ceil(64);
        let mut placements = Vec::new();

        for orientation in orientations {
            if orientation.width > region_width || orientation.height > region_height {
                continue;
            }

            for offset_y in 0..=(region_height - orientation.height) {
                for offset_x in 0..=(region_width - orientation.width) {
                    let mut bits = vec![0_u64; word_count];

                    for &(cell_x, cell_y) in &orientation.cells {
                        let x = offset_x + cell_x;
                        let y = offset_y + cell_y;
                        let index = y * region_width + x;
                        bits[index / 64] |= 1_u64 << (index % 64);
                    }

                    placements.push(Placement { bits });
                }
            }
        }

        placements
    }

    fn overlaps(occupied: &[u64], placement: &[u64]) -> bool {
        occupied
            .iter()
            .zip(placement.iter())
            .any(|(&left, &right)| (left & right) != 0)
    }

    fn place(occupied: &mut [u64], placement: &[u64]) {
        for (dst, &src) in occupied.iter_mut().zip(placement.iter()) {
            *dst |= src;
        }
    }

    fn unplace(occupied: &mut [u64], placement: &[u64]) {
        for (dst, &src) in occupied.iter_mut().zip(placement.iter()) {
            *dst &= !src;
        }
    }

    fn search(
        shapes: &[ShapeData],
        counts: &mut [usize],
        next_start: &mut [usize],
        occupied: &mut [u64],
        remaining_area: usize,
        region_area: usize,
        used_area: usize,
    ) -> bool {
        if remaining_area == 0 {
            return true;
        }

        if region_area - used_area < remaining_area {
            return false;
        }

        let mut best_shape_index = None;
        let mut best_count = usize::MAX;
        let mut best_candidates = Vec::new();

        for (shape_index, shape) in shapes.iter().enumerate() {
            if counts[shape_index] == 0 {
                continue;
            }

            let mut candidates = Vec::new();

            for placement_index in next_start[shape_index]..shape.placements.len() {
                let placement = &shape.placements[placement_index];
                if !overlaps(occupied, &placement.bits) {
                    candidates.push(placement_index);
                }
            }

            if candidates.len() < counts[shape_index] {
                return false;
            }

            if candidates.is_empty() {
                return false;
            }

            if candidates.len() < best_count {
                best_count = candidates.len();
                best_shape_index = Some(shape_index);
                best_candidates = candidates;
            }
        }

        let Some(shape_index) = best_shape_index else {
            return false;
        };

        let previous_start = next_start[shape_index];
        counts[shape_index] -= 1;

        for placement_index in best_candidates {
            let placement = &shapes[shape_index].placements[placement_index];
            place(occupied, &placement.bits);
            next_start[shape_index] = placement_index + 1;

            if search(
                shapes,
                counts,
                next_start,
                occupied,
                remaining_area - shapes[shape_index].area,
                region_area,
                used_area + shapes[shape_index].area,
            ) {
                next_start[shape_index] = previous_start;
                counts[shape_index] += 1;
                unplace(occupied, &placement.bits);
                return true;
            }

            unplace(occupied, &placement.bits);
        }

        next_start[shape_index] = previous_start;
        counts[shape_index] += 1;
        false
    }

    let (all_orientations, shape_areas, regions) = parse_input(s);
    let mut answer = 0_usize;

    for region in regions {
        if region.counts.len() != all_orientations.len() {
            continue;
        }

        let total_required_area = region
            .counts
            .iter()
            .zip(shape_areas.iter())
            .map(|(&count, &area)| count * area)
            .sum::<usize>();

        let region_area = region.width * region.height;
        if total_required_area > region_area {
            continue;
        }

        let mut shapes = Vec::new();
        let mut counts = Vec::new();

        for (shape_index, &count) in region.counts.iter().enumerate() {
            if count == 0 {
                continue;
            }

            let placements = build_placements(
                &all_orientations[shape_index],
                region.width,
                region.height,
            );

            if placements.is_empty() {
                shapes.clear();
                counts.clear();
                break;
            }

            shapes.push(ShapeData {
                area: shape_areas[shape_index],
                placements,
            });
            counts.push(count);
        }

        if total_required_area == 0 {
            answer += 1;
            continue;
        }

        if shapes.is_empty() {
            continue;
        }

        let mut order = (0..shapes.len()).collect::<Vec<_>>();
        order.sort_unstable_by_key(|&index| {
            (
                shapes[index].placements.len(),
                std::cmp::Reverse(shapes[index].area),
                std::cmp::Reverse(counts[index]),
            )
        });

        let reordered_shapes = order
            .iter()
            .map(|&index| shapes[index].clone())
            .collect::<Vec<_>>();
        let mut reordered_counts = order.iter().map(|&index| counts[index]).collect::<Vec<_>>();

        let mut next_start = vec![0_usize; reordered_shapes.len()];
        let word_count = region_area.div_ceil(64);
        let mut occupied = vec![0_u64; word_count];

        if search(
            &reordered_shapes,
            &mut reordered_counts,
            &mut next_start,
            &mut occupied,
            total_required_area,
            region_area,
            0,
        ) {
            answer += 1;
        }
    }

    answer
}

pub fn d12p1_v2(s: &str) -> usize {
    #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Orientation {
        cells: Vec<(usize, usize)>,
        width: usize,
        height: usize,
    }

    #[derive(Clone, Debug)]
    struct Placement {
        board_bits: Vec<u64>,
    }

    #[derive(Clone, Debug)]
    struct ShapeData {
        area: usize,
        placements: Vec<Placement>,
        placement_scores: Vec<usize>,
        compat: Vec<Vec<Vec<u64>>>,
        live_words: usize,
    }

    #[derive(Clone, Debug)]
    struct Region {
        width: usize,
        height: usize,
        counts: Vec<usize>,
    }

    #[derive(Clone, Debug)]
    struct WordChange {
        shape_index: usize,
        word_index: usize,
        previous: u64,
    }

    fn is_shape_row(line: &str) -> bool {
        !line.is_empty() && line.bytes().all(|byte| byte == b'#' || byte == b'.')
    }

    fn parse_shape_header(line: &str) -> Option<usize> {
        let trimmed = line.trim();
        let (left, right) = trimmed.split_once(':')?;
        if !right.trim().is_empty() {
            return None;
        }
        left.trim().parse::<usize>().ok()
    }

    fn parse_region(line: &str) -> Option<Region> {
        let trimmed = line.trim();
        let (dims, counts_part) = trimmed.split_once(':')?;
        let (width_str, height_str) = dims.trim().split_once('x')?;
        let width = width_str.trim().parse::<usize>().ok()?;
        let height = height_str.trim().parse::<usize>().ok()?;
        let counts = counts_part
            .split_whitespace()
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        Some(Region {
            width,
            height,
            counts,
        })
    }

    fn normalize(points: &[(isize, isize)]) -> Orientation {
        let min_x = points.iter().map(|&(x, _)| x).min().unwrap_or(0);
        let min_y = points.iter().map(|&(_, y)| y).min().unwrap_or(0);

        let mut cells = points
            .iter()
            .map(|&(x, y)| {
                let nx = usize::try_from(x - min_x)
                    .expect("normalized x coordinate must be non-negative");
                let ny = usize::try_from(y - min_y)
                    .expect("normalized y coordinate must be non-negative");
                (nx, ny)
            })
            .collect::<Vec<_>>();

        cells.sort_unstable();

        let width = cells
            .iter()
            .map(|&(x, _)| x)
            .max()
            .map_or(0, |value| value + 1);

        let height = cells
            .iter()
            .map(|&(_, y)| y)
            .max()
            .map_or(0, |value| value + 1);

        Orientation {
            cells,
            width,
            height,
        }
    }

    fn make_orientations(shape_rows: &[String]) -> Vec<Orientation> {
        let mut base_points = Vec::new();

        for (y, row) in shape_rows.iter().enumerate() {
            for (x, byte) in row.bytes().enumerate() {
                if byte == b'#' {
                    base_points.push((
                        isize::try_from(x).expect("x must fit into isize"),
                        isize::try_from(y).expect("y must fit into isize"),
                    ));
                }
            }
        }

        let transforms: [fn(isize, isize) -> (isize, isize); 8] = [
            |x, y| (x, y),
            |x, y| (x, -y),
            |x, y| (-x, y),
            |x, y| (-x, -y),
            |x, y| (y, x),
            |x, y| (y, -x),
            |x, y| (-y, x),
            |x, y| (-y, -x),
        ];

        let mut orientations = Vec::new();

        for transform in transforms {
            let transformed = base_points
                .iter()
                .map(|&(x, y)| transform(x, y))
                .collect::<Vec<_>>();

            let normalized = normalize(&transformed);

            if !orientations.contains(&normalized) {
                orientations.push(normalized);
            }
        }

        orientations
    }

    fn parse_input(input: &str) -> (Vec<Vec<Orientation>>, Vec<usize>, Vec<Region>) {
        let mut raw_shapes: Vec<Option<Vec<String>>> = Vec::new();
        let mut regions = Vec::new();
        let mut lines = input.lines().peekable();

        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if let Some(index) = parse_shape_header(trimmed) {
                let mut rows = Vec::new();

                while let Some(next_line) = lines.peek() {
                    let next_trimmed = next_line.trim();
                    if !is_shape_row(next_trimmed) {
                        break;
                    }
                    rows.push(next_trimmed.to_owned());
                    let _ = lines.next();
                }

                if raw_shapes.len() <= index {
                    raw_shapes.resize_with(index + 1, || None);
                }
                raw_shapes[index] = Some(rows);
            } else if let Some(region) = parse_region(trimmed) {
                regions.push(region);
            }
        }

        let mut shapes = Vec::new();
        let mut shape_areas = Vec::new();

        for maybe_rows in raw_shapes {
            let rows = maybe_rows.expect("shape indices must be contiguous");
            let orientations = make_orientations(&rows);
            let area = orientations
                .first()
                .map(|orientation| orientation.cells.len())
                .unwrap_or(0);

            shapes.push(orientations);
            shape_areas.push(area);
        }

        (shapes, shape_areas, regions)
    }

    fn overlaps_bits(left: &[u64], right: &[u64]) -> bool {
        left.iter()
            .zip(right.iter())
            .any(|(&left_word, &right_word)| (left_word & right_word) != 0)
    }

    fn build_placements(
        orientations: &[Orientation],
        region_width: usize,
        region_height: usize,
    ) -> Vec<Placement> {
        let cell_count = region_width * region_height;
        let board_word_count = cell_count.div_ceil(64);
        let mut placements = Vec::new();

        for orientation in orientations {
            if orientation.width > region_width || orientation.height > region_height {
                continue;
            }

            for offset_y in 0..=(region_height - orientation.height) {
                for offset_x in 0..=(region_width - orientation.width) {
                    let mut board_bits = vec![0_u64; board_word_count];

                    for &(cell_x, cell_y) in &orientation.cells {
                        let x = offset_x + cell_x;
                        let y = offset_y + cell_y;
                        let index = y * region_width + x;
                        board_bits[index / 64] |= 1_u64 << (index % 64);
                    }

                    placements.push(Placement { board_bits });
                }
            }
        }

        placements
    }

    fn set_bit(bits: &mut [u64], index: usize) {
        bits[index / 64] |= 1_u64 << (index % 64);
    }

    fn popcount_bits(bits: &[u64]) -> usize {
        bits.iter()
            .map(|word| usize::try_from(word.count_ones()).expect("count_ones must fit usize"))
            .sum()
    }

    fn iter_set_bits(bits: &[u64], mut visitor: impl FnMut(usize)) {
        for (word_index, &word) in bits.iter().enumerate() {
            let mut remaining = word;
            while remaining != 0 {
                let bit =
                    usize::try_from(remaining.trailing_zeros()).expect("bit index must fit usize");
                visitor(word_index * 64 + bit);
                remaining &= remaining - 1;
            }
        }
    }

    fn apply_mask_in_place(
        live: &mut [Vec<u64>],
        shape_index: usize,
        mask: &[u64],
        trail: &mut Vec<WordChange>,
    ) {
        for (word_index, (live_word, &mask_word)) in
            live[shape_index].iter_mut().zip(mask.iter()).enumerate()
        {
            let new_word = *live_word & mask_word;
            if new_word != *live_word {
                trail.push(WordChange {
                    shape_index,
                    word_index,
                    previous: *live_word,
                });
                *live_word = new_word;
            }
        }
    }

    fn apply_keep_strictly_after(
        live: &mut [Vec<u64>],
        shape_index: usize,
        placement_index: usize,
        trail: &mut Vec<WordChange>,
    ) {
        let next_allowed = placement_index + 1;
        let live_words = live[shape_index].len();
        let full_words_to_zero = next_allowed / 64;
        let bit_offset = next_allowed % 64;

        for word_index in 0..full_words_to_zero.min(live_words) {
            let current = live[shape_index][word_index];
            if current != 0 {
                trail.push(WordChange {
                    shape_index,
                    word_index,
                    previous: current,
                });
                live[shape_index][word_index] = 0;
            }
        }

        if full_words_to_zero < live_words && bit_offset != 0 {
            let keep_mask = !((1_u64 << bit_offset) - 1);
            let current = live[shape_index][full_words_to_zero];
            let new_word = current & keep_mask;
            if new_word != current {
                trail.push(WordChange {
                    shape_index,
                    word_index: full_words_to_zero,
                    previous: current,
                });
                live[shape_index][full_words_to_zero] = new_word;
            }
        }
    }

    fn rollback(live: &mut [Vec<u64>], trail: &mut Vec<WordChange>, checkpoint: usize) {
        while trail.len() > checkpoint {
            let change = trail.pop().expect("trail must contain a change");
            live[change.shape_index][change.word_index] = change.previous;
        }
    }

    fn choose_shape(shapes: &[ShapeData], counts: &[usize], live: &[Vec<u64>]) -> Option<usize> {
        let mut best_shape_index = None;
        let mut best_available = usize::MAX;
        let mut best_score = usize::MAX;

        for (shape_index, shape) in shapes.iter().enumerate() {
            if counts[shape_index] == 0 {
                continue;
            }

            let available = popcount_bits(&live[shape_index]);
            if available < counts[shape_index] {
                return Some(shape_index);
            }

            let score = available.saturating_sub(counts[shape_index]);

            if available < best_available || (available == best_available && score < best_score) {
                best_available = available;
                best_score = score;
                best_shape_index = Some(shape_index);
            }
        }

        best_shape_index
    }

    fn collect_sorted_candidates(
        shape: &ShapeData,
        live_bits: &[u64],
        buffer: &mut Vec<usize>,
    ) {
        buffer.clear();
        iter_set_bits(live_bits, |placement_index| {
            if placement_index < shape.placements.len() {
                buffer.push(placement_index);
            }
        });
        buffer.sort_unstable_by_key(|&placement_index| shape.placement_scores[placement_index]);
    }

    fn search(
        shapes: &[ShapeData],
        counts: &mut [usize],
        live: &mut [Vec<u64>],
        remaining_area: usize,
        region_area: usize,
        used_area: usize,
        trail: &mut Vec<WordChange>,
        candidate_buffer: &mut Vec<usize>,
    ) -> bool {
        if remaining_area == 0 {
            return true;
        }

        if region_area - used_area < remaining_area {
            return false;
        }

        let Some(shape_index) = choose_shape(shapes, counts, live) else {
            return false;
        };

        let available = popcount_bits(&live[shape_index]);
        if available < counts[shape_index] || available == 0 {
            return false;
        }

        collect_sorted_candidates(&shapes[shape_index], &live[shape_index], candidate_buffer);
        let candidates = candidate_buffer.clone();

        counts[shape_index] -= 1;

        for placement_index in candidates {
            let checkpoint = trail.len();

            for (other_shape_index, mask) in shapes[shape_index].compat[placement_index]
                .iter()
                .enumerate()
            {
                apply_mask_in_place(live, other_shape_index, mask, trail);
            }

            apply_keep_strictly_after(live, shape_index, placement_index, trail);

            let mut viable = true;
            for (other_shape_index, &remaining_count) in counts.iter().enumerate() {
                if remaining_count == 0 {
                    continue;
                }
                if popcount_bits(&live[other_shape_index]) < remaining_count {
                    viable = false;
                    break;
                }
            }

            if viable
                && search(
                    shapes,
                    counts,
                    live,
                    remaining_area - shapes[shape_index].area,
                    region_area,
                    used_area + shapes[shape_index].area,
                    trail,
                    candidate_buffer,
                )
            {
                counts[shape_index] += 1;
                rollback(live, trail, checkpoint);
                return true;
            }

            rollback(live, trail, checkpoint);
        }

        counts[shape_index] += 1;
        false
    }

    fn build_shape_data_for_region(
        all_orientations: &[Vec<Orientation>],
        shape_areas: &[usize],
        region: &Region,
    ) -> Option<(Vec<ShapeData>, Vec<usize>, usize)> {
        if region.counts.len() != all_orientations.len() {
            return None;
        }

        let total_required_area = region
            .counts
            .iter()
            .zip(shape_areas.iter())
            .map(|(&count, &area)| count * area)
            .sum::<usize>();

        let region_area = region.width * region.height;
        if total_required_area > region_area {
            return None;
        }

        let active_indices = region
            .counts
            .iter()
            .enumerate()
            .filter_map(|(index, &count)| (count > 0).then_some(index))
            .collect::<Vec<_>>();

        let mut shapes = Vec::with_capacity(active_indices.len());
        let mut counts = Vec::with_capacity(active_indices.len());

        for &shape_index in &active_indices {
            let placements = build_placements(
                &all_orientations[shape_index],
                region.width,
                region.height,
            );

            if placements.is_empty() {
                return None;
            }

            let live_words = placements.len().div_ceil(64);

            shapes.push(ShapeData {
                area: shape_areas[shape_index],
                placements,
                placement_scores: Vec::new(),
                compat: Vec::new(),
                live_words,
            });
            counts.push(region.counts[shape_index]);
        }

        let shape_count = shapes.len();

        for source_shape_index in 0..shape_count {
            let placement_count = shapes[source_shape_index].placements.len();
            let mut compat_for_shape = Vec::with_capacity(placement_count);
            let mut scores = Vec::with_capacity(placement_count);

            for placement_index in 0..placement_count {
                let source_placement = &shapes[source_shape_index].placements[placement_index];
                let mut per_target_shape = Vec::with_capacity(shape_count);
                let mut score = 0_usize;

                for target_shape_index in 0..shape_count {
                    let target_placements = &shapes[target_shape_index].placements;
                    let target_live_words = shapes[target_shape_index].live_words;
                    let mut mask = vec![0_u64; target_live_words];

                    for (target_placement_index, target_placement) in target_placements.iter().enumerate() {
                        if !overlaps_bits(&source_placement.board_bits, &target_placement.board_bits)
                        {
                            set_bit(&mut mask, target_placement_index);
                            score += 1;
                        }
                    }

                    per_target_shape.push(mask);
                }

                compat_for_shape.push(per_target_shape);
                scores.push(score);
            }

            shapes[source_shape_index].compat = compat_for_shape;
            shapes[source_shape_index].placement_scores = scores;
        }

        Some((shapes, counts, total_required_area))
    }

    let (all_orientations, shape_areas, regions) = parse_input(s);
    let mut answer = 0_usize;

    for region in regions {
        let Some((shapes, mut counts, total_required_area)) =
            build_shape_data_for_region(&all_orientations, &shape_areas, &region)
        else {
            continue;
        };

        if total_required_area == 0 {
            answer += 1;
            continue;
        }

        let mut live = shapes
            .iter()
            .map(|shape| {
                let mut bits = vec![u64::MAX; shape.live_words];
                let extra = shape.live_words * 64 - shape.placements.len();
                if extra > 0 {
                    let keep_bits = 64 - extra;
                    let mask = if keep_bits == 64 {
                        u64::MAX
                    } else {
                        (1_u64 << keep_bits) - 1
                    };
                    let last_index = shape.live_words - 1;
                    bits[last_index] = mask;
                }
                bits
            })
            .collect::<Vec<_>>();

        let mut trail = Vec::new();
        let mut candidate_buffer = Vec::new();
        let region_area = region.width * region.height;

        if search(
            &shapes,
            &mut counts,
            &mut live,
            total_required_area,
            region_area,
            0,
            &mut trail,
            &mut candidate_buffer,
        ) {
            answer += 1;
        }
    }

    answer
}

#[allow(unused)]
pub fn d12p1(s: &str) -> usize {
    d12p1_v2(s)
}

#[cfg(test)]
mod tests {
    use crate::d12::{d12p1};

    #[test]
    fn d12p1_test() {
        let s = include_str!("d12_test.txt");
        let result: usize = d12p1(s);
        println!("result: {}", result);
        assert_eq!(2, result);
    }
}
