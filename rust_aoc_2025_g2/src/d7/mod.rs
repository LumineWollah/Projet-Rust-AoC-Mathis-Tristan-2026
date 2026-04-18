use std::collections::{BTreeMap, HashSet};

// helper commun aux trois versions : on parse le grid et on trouve la position
// du 'S' qui est le point de départ du faisceau initial
fn parse_grid_and_start(s: &str) -> (Vec<Vec<char>>, usize, usize) {
    let grid: Vec<Vec<char>> = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    let mut start_x = 0usize;
    let mut start_y = 0usize;
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'S' {
                start_x = x;
                start_y = y;
            }
        }
    }
    (grid, start_x, start_y)
}

// petite struct utilisée par les versions de la partie 1
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

// version 1 (partie 1) : tentative naïve sans cache.
// chaque faisceau qui descend déclenche le splitter rencontré, qui crée 2 nouveaux
// faisceaux, et on traite la queue en BFS. problème : un même splitter peut être
// re-traité plein de fois -> "boucle inf" sur les vraies données.
//
// pour pouvoir quand même valider la fonction sur le fichier de test sans bloquer
// les benchs, on borne le nombre d'itérations. la borne est très généreuse pour
// laisser passer le test mais éviter une explosion sur le vrai input.
#[allow(unused)]
pub fn d7p1_v1(s: &str) -> u64 {
    let (grid, start_x, start_y) = parse_grid_and_start(s);
    let height = grid.len();
    if height == 0 {
        return 0;
    }
    let width = grid[0].len();

    let mut queue: Vec<Pos> = Vec::new();
    queue.push(Pos { x: start_x, y: start_y });

    let mut count = 0u64;
    let mut head = 0usize;

    // garde-fou pour éviter l'explosion combinatoire de la version naïve
    let max_iter: usize = 1_000_000;
    let mut iter = 0usize;

    while head < queue.len() && iter < max_iter {
        iter += 1;
        let current_x = queue[head].x;
        let current_y = queue[head].y;
        head += 1;

        // le faisceau descend tout droit
        for y in (current_y + 1)..height {
            if grid[y][current_x] == '^' {
                count += 1;

                // gauche
                if current_x > 0 {
                    queue.push(Pos { x: current_x - 1, y });
                }
                // droite
                if current_x < width - 1 {
                    queue.push(Pos { x: current_x + 1, y });
                }
                break;
            }
        }
    }

    count
}

// version 2 (partie 1) : on ajoute un HashSet de splitters déjà traités.
// chaque splitter ne contribue qu'une seule fois au compteur, ce qui résout
// l'explosion vue dans la v1.
#[allow(unused)]
pub fn d7p1_v2(s: &str) -> u64 {
    let (grid, start_x, start_y) = parse_grid_and_start(s);
    let height = grid.len();
    if height == 0 {
        return 0;
    }
    let width = grid[0].len();

    let start = Pos { x: start_x, y: start_y };
    let mut queue: Vec<Pos> = Vec::new();
    queue.push(start);

    // le cache pour pas traiter 50 fois le même splitter
    let mut visited_splitters: HashSet<Pos> = HashSet::new();
    let mut count = 0u64;
    let mut head = 0usize;

    while head < queue.len() {
        let curr = queue[head];
        head += 1;

        for y in (curr.y + 1)..height {
            if grid[y][curr.x] == '^' {
                let splitter_pos = Pos { x: curr.x, y };

                // si on l'a déjà split celui-là, on s'arrête
                if visited_splitters.contains(&splitter_pos) {
                    break;
                }

                // sinon on le marque et on continue
                visited_splitters.insert(splitter_pos);
                count += 1;

                if curr.x > 0 {
                    queue.push(Pos { x: curr.x - 1, y });
                }
                if curr.x < width - 1 {
                    queue.push(Pos { x: curr.x + 1, y });
                }

                break;
            }
        }
    }

    count
}

// petite struct utilisée par la partie 2 où on doit ordonner par y croissant
// pour fusionner les timelines qui arrivent au même splitter
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PosOrdered {
    // trier par y en premier, c'est mieux pour descendre niveau par niveau
    y: usize,
    x: usize,
}

// version 1 (partie 2) : on ne compte plus les splitters mais le nombre total
// de timelines qui arrivent en bas du grid.
// idée : un BTreeMap (Pos -> nombre de timelines) trié par y. on dépile toujours
// le plus haut. à chaque splitter rencontré, on ajoute `timelines` aux deux voisins
// gauche/droit qui se trouvent juste sous le splitter. si un faisceau ne tape
// aucun splitter, ses timelines comptent dans le total final.
#[allow(unused)]
pub fn d7p2_v1(s: &str) -> u64 {
    let (grid, start_x, start_y) = parse_grid_and_start(s);
    let height = grid.len();
    if height == 0 {
        return 0;
    }
    let width = grid[0].len();

    // on regroupe les timelines par splitter de départ
    let mut splitters: BTreeMap<PosOrdered, u64> = BTreeMap::new();

    // au début, on a 1 seule timeline qui part de S
    splitters.insert(PosOrdered { x: start_x, y: start_y }, 1);

    let mut total_final_timelines = 0u64;

    // on traite les splitters ligne par ligne (le BTreeMap nous donne le plus haut en premier)
    while let Some((curr_pos, timelines)) = splitters.pop_first() {
        let mut hit_splitter = false;

        // le faisceau descend
        for y in (curr_pos.y + 1)..height {
            if grid[y][curr_pos.x] == '^' {
                // chaque timeline actuelle se divise en 2
                // mais elles arrivent sur le même splitter

                // timeline gauche
                if curr_pos.x > 0 {
                    let left = PosOrdered { x: curr_pos.x - 1, y };
                    *splitters.entry(left).or_insert(0) += timelines;
                }

                // timeline droite
                if curr_pos.x < width - 1 {
                    let right = PosOrdered { x: curr_pos.x + 1, y };
                    *splitters.entry(right).or_insert(0) += timelines;
                }

                hit_splitter = true;
                break;
            }
        }

        // chaque timeline qui sort du grid sans rencontrer de splitter compte pour le total
        if !hit_splitter {
            total_final_timelines += timelines;
        }
    }

    total_final_timelines
}

#[allow(unused)]
pub fn d7p1(s: &str) -> u64 {
    d7p1_v2(s)
}

#[allow(unused)]
pub fn d7p2(s: &str) -> u64 {
    d7p2_v1(s)
}

#[cfg(test)]
mod tests {
    use crate::d7::{d7p1, d7p2};

    #[test]
    fn d7p1_test() {
        let s = include_str!("d7_test.txt");
        let result = d7p1(s);
        println!("result: {}", result);
        // nombre de splitters uniques atteints (chaque '^' n'est compté qu'une fois)
        // sur le grid pyramidal du fichier de test
        assert_eq!(21, result);
    }

    #[test]
    fn d7p2_test() {
        let s = include_str!("d7_test.txt");
        let result = d7p2(s);
        println!("result: {}", result);
        // chaque splitter double les timelines qui le traversent, et on additionne
        // toutes les timelines qui sortent en bas du grid sans toucher de splitter.
        assert_eq!(40, result);
    }
}
