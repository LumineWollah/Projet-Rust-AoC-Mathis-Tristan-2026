#[allow(unused)]
pub fn d8p1_v1(s: &str, connections: usize) -> usize {
    #[derive(Clone, Copy)]
    struct Point {
        x: i64,
        y: i64,
        z: i64,
    }

    fn squared_distance(a: Point, b: Point) -> i64 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dz = a.z - b.z;
        dx * dx + dy * dy + dz * dz
    }

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut [usize], size: &mut [usize], a: usize, b: usize) {
        let root_a = find(parent, a);
        let root_b = find(parent, b);

        if root_a == root_b {
            return;
        }

        parent[root_b] = root_a;
        size[root_a] += size[root_b];
    }

    let points: Vec<Point> = s
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            let z = parts.next().unwrap().parse::<i64>().unwrap();

            Point { x, y, z }
        })
        .collect();

    let n = points.len();
    if n < 3 {
        return 0;
    }

    let mut pairs: Vec<(i64, usize, usize)> = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let dist = squared_distance(points[i], points[j]);
            pairs.push((dist, i, j));
        }
    }

    pairs.sort_by_key(|&(dist, _, _)| dist);

    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];

    let limit = connections.min(pairs.len());
    for &(_, a, b) in pairs.iter().take(limit) {
        union(&mut parent, &mut size, a, b);
    }

    let mut circuit_sizes: Vec<usize> = Vec::new();
    for (i, &component_size) in size.iter().enumerate() {
        let root = find(&mut parent, i);
        if root == i {
            circuit_sizes.push(component_size);
        }
    }

    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));

    if circuit_sizes.len() < 3 {
        return 0;
    }

    circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]
}

#[allow(unused)]
pub fn d8p1_v2(s: &str, connections: usize) -> usize {
    // Optimisation: preallocate the pairs vector, use sort_unstable_by_key, and use union-by-size

    #[derive(Clone, Copy)]
    struct Point {
        x: i64,
        y: i64,
        z: i64,
    }

    fn squared_distance(a: Point, b: Point) -> i64 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dz = a.z - b.z;
        dx * dx + dy * dy + dz * dz
    }

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut [usize], size: &mut [usize], a: usize, b: usize) {
        let mut root_a = find(parent, a);
        let mut root_b = find(parent, b);

        if root_a == root_b {
            return;
        }

        if size[root_a] < size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }

        parent[root_b] = root_a;
        size[root_a] += size[root_b];
    }

    let points: Vec<Point> = s
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            let z = parts.next().unwrap().parse::<i64>().unwrap();

            Point { x, y, z }
        })
        .collect();

    let n = points.len();
    if n < 3 {
        return 0;
    }

    let pair_count = n * (n - 1) / 2;
    let mut pairs: Vec<(i64, usize, usize)> = Vec::with_capacity(pair_count);

    for i in 0..n {
        for j in (i + 1)..n {
            let dist = squared_distance(points[i], points[j]);
            pairs.push((dist, i, j));
        }
    }

    pairs.sort_unstable_by_key(|&(dist, _, _)| dist);

    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];

    let limit = connections.min(pairs.len());
    for &(_, a, b) in pairs.iter().take(limit) {
        union(&mut parent, &mut size, a, b);
    }

    let mut circuit_sizes: Vec<usize> = Vec::new();
    for (i, &component_size) in size.iter().enumerate() {
        let root = find(&mut parent, i);
        if root == i {
            circuit_sizes.push(component_size);
        }
    }
    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));

    if circuit_sizes.len() < 3 {
        return 0;
    }

    circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]
}

#[allow(unused)]
pub fn d8p2_v1(s: &str) -> usize {
    #[derive(Clone, Copy)]
    struct Point {
        x: i64,
        y: i64,
        z: i64,
    }

    fn squared_distance(a: Point, b: Point) -> i64 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dz = a.z - b.z;
        dx * dx + dy * dy + dz * dz
    }

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut [usize], size: &mut [usize], a: usize, b: usize) -> bool {
        let root_a = find(parent, a);
        let root_b = find(parent, b);

        if root_a == root_b {
            return false;
        }

        parent[root_b] = root_a;
        size[root_a] += size[root_b];
        true
    }

    let points: Vec<Point> = s
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            let z = parts.next().unwrap().parse::<i64>().unwrap();

            Point { x, y, z }
        })
        .collect();

    let n = points.len();
    if n < 2 {
        return 0;
    }

    let mut pairs: Vec<(i64, usize, usize)> = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let dist = squared_distance(points[i], points[j]);
            pairs.push((dist, i, j));
        }
    }

    pairs.sort_by_key(|&(dist, _, _)| dist);

    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];
    let mut components = n;

    for &(_, a, b) in &pairs {
        if union(&mut parent, &mut size, a, b) {
            components -= 1;

            if components == 1 {
                let product = points[a].x * points[b].x;
                return usize::try_from(product).unwrap_or(0);
            }
        }
    }

    0
}

#[allow(unused)]
pub fn d8p2_v2(s: &str) -> usize {
    // Optimisation: preallocate the pairs vector, use sort_unstable_by_key, and use union-by-size

    #[derive(Clone, Copy)]
    struct Point {
        x: i64,
        y: i64,
        z: i64,
    }

    fn squared_distance(a: Point, b: Point) -> i64 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dz = a.z - b.z;
        dx * dx + dy * dy + dz * dz
    }

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut [usize], size: &mut [usize], a: usize, b: usize) -> bool {
        let mut root_a = find(parent, a);
        let mut root_b = find(parent, b);

        if root_a == root_b {
            return false;
        }

        if size[root_a] < size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }

        parent[root_b] = root_a;
        size[root_a] += size[root_b];
        true
    }

    let points: Vec<Point> = s
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            let z = parts.next().unwrap().parse::<i64>().unwrap();

            Point { x, y, z }
        })
        .collect();

    let n = points.len();
    if n < 2 {
        return 0;
    }

    let pair_count = n * (n - 1) / 2;
    let mut pairs: Vec<(i64, usize, usize)> = Vec::with_capacity(pair_count);

    for i in 0..n {
        for j in (i + 1)..n {
            let dist = squared_distance(points[i], points[j]);
            pairs.push((dist, i, j));
        }
    }

    pairs.sort_unstable_by_key(|&(dist, _, _)| dist);

    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];
    let mut components = n;

    for &(_, a, b) in &pairs {
        if union(&mut parent, &mut size, a, b) {
            components -= 1;

            if components == 1 {
                let product = points[a].x * points[b].x;
                return usize::try_from(product).unwrap_or(0);
            }
        }
    }

    0
}

#[allow(unused)]
pub fn d8p1(s: &str, connections: usize) -> usize {
    d8p1_v2(s, connections)
}

#[allow(unused)]
pub fn d8p2(s: &str) -> usize {
    d8p2_v2(s)
}

#[cfg(test)]
mod tests {
    use crate::d8::{d8p1, d8p2};

    #[test]
    fn d8p1_test() {
        let s = include_str!("d8_test.txt");
        let result = d8p1(s,10);
        println!("result: {result}");
        assert_eq!(40, result);
    }

    #[test]
    fn d8p2_test() {
        let s = include_str!("d8_test.txt");
        let result = d8p2(s);
        println!("result: {result}");
        assert_eq!(25_272, result);
    }
}