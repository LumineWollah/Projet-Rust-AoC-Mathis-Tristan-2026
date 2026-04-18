// helper commun : on parse les points "x,y" un par ligne en ignorant les vides
fn parse_points(s: &str) -> Vec<(i64, i64)> {
    let mut points: Vec<(i64, i64)> = Vec::new();
    for line in s.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let coords: Vec<&str> = line.split(',').collect();
        let x = coords[0].trim().parse::<i64>().unwrap();
        let y = coords[1].trim().parse::<i64>().unwrap();
        points.push((x, y));
    }
    points
}

// version 1 (partie 1) : aire de la bounding box de chaque paire de points,
// version i32 sans le +1. souffrait d'overflow sur les vraies données et
// le +1 manquant fait qu'on ne compte pas les bordures.
#[allow(unused)]
pub fn d9p1_v1(s: &str) -> i64 {
    fn area_calculator(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
        let width = (x2 - x1).abs();
        let height = (y2 - y1).abs();
        return width * height;
    }

    let points: Vec<(i32, i32)> = s
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let coords: Vec<&str> = l.split(',').collect();
            let x = coords[0].trim().parse::<i32>().unwrap();
            let y = coords[1].trim().parse::<i32>().unwrap();
            (x, y)
        })
        .collect();

    let mut highest_area: i32 = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let area = area_calculator(x1, y1, x2, y2);
            if area > highest_area {
                highest_area = area;
            }
        }
    }
    i64::from(highest_area)
}

// version 2 (partie 1) : on passe en i64 pour éviter l'overflow, mais on a
// toujours pas le +1 sur la formule donc le résultat reste faux d'un peu.
#[allow(unused)]
pub fn d9p1_v2(s: &str) -> i64 {
    fn area_calculator(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
        let width = (x2 - x1).abs();
        let height = (y2 - y1).abs();
        return width * height;
    }

    let points = parse_points(s);

    let mut highest_area: i64 = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let area = area_calculator(x1, y1, x2, y2);
            if area > highest_area {
                highest_area = area;
            }
        }
    }
    highest_area
}

// version 3 (partie 1) : on ajoute le +1 dans la formule pour bien compter
// les tuiles aux bornes inclusives. ça donne enfin le bon résultat de l'étape 1.
#[allow(unused)]
pub fn d9p1_v3(s: &str) -> i64 {
    fn area_calculator(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
        // voilà juste un bolosse
        let width = (x2 - x1).abs() + 1;
        let height = (y2 - y1).abs() + 1;
        return width * height;
    }

    let points = parse_points(s);

    let mut highest_area: i64 = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let area = area_calculator(x1, y1, x2, y2);
            if area > highest_area {
                highest_area = area;
            }
        }
    }
    highest_area
}

// fonction utilitaire pour la partie 2 :
// renvoie true si le point (x,y) est sur une bordure du polygone OU dedans.
// implémenté avec un ray casting horizontal classique + détection des points
// alignés sur un segment horizontal/vertical.
fn est_autorise(x: i64, y: i64, points: &[(i64, i64)]) -> bool {
    let mut dedans = false;
    let n = points.len();
    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n];

        // verifie si le point est sur le segment (bordure)
        if (p1.0 == p2.0 && p1.0 == x && y >= p1.1.min(p2.1) && y <= p1.1.max(p2.1))
            || (p1.1 == p2.1 && p1.1 == y && x >= p1.0.min(p2.0) && x <= p1.0.max(p2.0))
        {
            return true;
        }

        // algorithme de ray casting pour l interieur
        if ((p1.1 > y) != (p2.1 > y))
            && (x < (p2.0 - p1.0) * (y - p1.1) / (p2.1 - p1.1) + p1.0)
        {
            dedans = !dedans;
        }
    }
    dedans
}

// version 1 (partie 2) : pour chaque paire de points, on vérifie chaque tuile
// du rectangle borne par cette paire pour voir si elle est dans le polygone.
// très lent dès que le polygone est grand (boucle imbriquée O(n^2 * w * h)).
#[allow(unused)]
pub fn d9p2_v1(s: &str) -> i64 {
    let points = parse_points(s);
    if points.is_empty() {
        return 0;
    }

    let mut aire_max: i64 = 0;

    // test de chaque paire de points rouges
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            let mut rectangle_ok = true;

            // verifie si chaque tuile du rectangle est verte ou rouge
            'test: for x in min_x..=max_x {
                for y in min_y..=max_y {
                    if !est_autorise(x, y, &points) {
                        rectangle_ok = false;
                        break 'test;
                    }
                }
            }

            if rectangle_ok {
                let aire = (max_x - min_x + 1) * (max_y - min_y + 1);
                if aire > aire_max {
                    aire_max = aire;
                }
            }
        }
    }
    aire_max
}

// version 2 (partie 2) : on précalcule une grille booléenne (true = case dans
// le polygone) une seule fois, puis on consulte la grille au lieu de relancer
// est_autorise. plus rapide mais sur les vraies données c'est encore trop lent.
#[allow(unused)]
pub fn d9p2_v2(s: &str) -> i64 {
    let points = parse_points(s);
    if points.is_empty() {
        return 0;
    }

    // limites pour la grille
    let min_x_g = points.iter().map(|p| p.0).min().unwrap();
    let max_x_g = points.iter().map(|p| p.0).max().unwrap();
    let min_y_g = points.iter().map(|p| p.1).min().unwrap();
    let max_y_g = points.iter().map(|p| p.1).max().unwrap();

    // creation grille de booleens pour gagner du temps
    // on decale les index de min_x et min_y pour que ca commence a zero
    let largeur = (max_x_g - min_x_g + 1) as usize;
    let hauteur = (max_y_g - min_y_g + 1) as usize;
    let mut grille = vec![vec![false; hauteur]; largeur];

    // on remplit la grille une seule fois
    for x in min_x_g..=max_x_g {
        for y in min_y_g..=max_y_g {
            if est_autorise(x, y, &points) {
                grille[(x - min_x_g) as usize][(y - min_y_g) as usize] = true;
            }
        }
    }

    let mut aire_max: i64 = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            let mut rectangle_ok = true;

            // verifie si chaque tuile est vraie dans la grille
            'test: for x in min_x..=max_x {
                for y in min_y..=max_y {
                    // on utilise les index decales
                    if !grille[(x - min_x_g) as usize][(y - min_y_g) as usize] {
                        rectangle_ok = false;
                        break 'test;
                    }
                }
            }

            if rectangle_ok {
                let aire = (max_x - min_x + 1) * (max_y - min_y + 1);
                if aire > aire_max {
                    aire_max = aire;
                }
            }
        }
    }
    aire_max
}

// version 3 (partie 2) : version finale qui ne scanne plus le contenu du
// rectangle. on vérifie d'abord 4 coins + le centre, puis on regarde si
// un mur (segment du polygone) traverse l'intérieur du rectangle. si non,
// le rectangle est valide.
// beaucoup plus rapide car on évite la double boucle sur (w*h).
#[allow(unused)]
pub fn d9p2_v3(s: &str) -> i64 {
    let points = parse_points(s);
    if points.is_empty() {
        return 0;
    }

    let mut aire_max: i64 = 0;
    let n = points.len();

    // preparation des murs pour verifier les collisions
    let mut segments = Vec::new();
    for i in 0..n {
        segments.push((points[i], points[(i + 1) % n]));
    }

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            // on verifie si les coins et le milieu sont dedans
            if !est_autorise(min_x, min_y, &points)
                || !est_autorise(max_x, max_y, &points)
                || !est_autorise(min_x, max_y, &points)
                || !est_autorise(max_x, min_y, &points)
                || !est_autorise((min_x + max_x) / 2, (min_y + max_y) / 2, &points)
            {
                continue;
            }

            // on verifie si un mur traverse le rectangle
            let mut collision = false;
            for (p1, p2) in &segments {
                let mur_x_dans = p1.0 > min_x && p1.0 < max_x;
                let mur_y_dans = p1.1 > min_y && p1.1 < max_y;

                if p1.0 == p2.0 {
                    // mur vertical
                    if mur_x_dans && !(p1.1.min(p2.1) >= max_y || p1.1.max(p2.1) <= min_y) {
                        collision = true;
                        break;
                    }
                } else {
                    // mur horizontal
                    if mur_y_dans && !(p1.0.min(p2.0) >= max_x || p1.0.max(p2.0) <= min_x) {
                        collision = true;
                        break;
                    }
                }
            }

            if !collision {
                let aire = (max_x - min_x + 1) * (max_y - min_y + 1);
                if aire > aire_max {
                    aire_max = aire;
                }
            }
        }
    }
    aire_max
}

#[allow(unused)]
pub fn d9p1(s: &str) -> i64 {
    d9p1_v3(s)
}

#[allow(unused)]
pub fn d9p2(s: &str) -> i64 {
    d9p2_v3(s)
}

#[cfg(test)]
mod tests {
    use crate::d9::{d9p1, d9p2};

    #[test]
    fn d9p1_test() {
        let s = include_str!("d9_test.txt");
        let result = d9p1(s);
        println!("result: {}", result);
        // bounding box max parmi toutes les paires de points : (2,3)-(11,7)
        // -> (11-2+1) * (7-3+1) = 10 * 5 = 50 (et plusieurs autres pairs donnent aussi 50)
        assert_eq!(50, result);
    }

    #[test]
    fn d9p2_test() {
        let s = include_str!("d9_test.txt");
        let result = d9p2(s);
        println!("result: {}", result);
        // plus grand rectangle, dont les coins sont des sommets du polygone,
        // entièrement contenu : pair (2,3)-(9,5) -> 8*3 = 24
        assert_eq!(24, result);
    }
}
