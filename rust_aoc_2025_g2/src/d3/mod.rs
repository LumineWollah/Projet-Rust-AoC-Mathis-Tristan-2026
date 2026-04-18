// version 1 (partie 1) : on cherche les deux plus grands chiffres de chaque ligne.
// première tentative très naïve : on parcourt et on garde les deux meilleurs caractères vus.
// bug connu : on retourne `first as i32` au lieu de l'entier formé par "first|second",
// donc on additionne en réalité les codes ASCII. on garde la fonction telle quelle pour
// montrer l'évolution de la réflexion.
#[allow(unused)]
pub fn d3p1_v1(s: &str) -> i64 {
    fn found_best_nbr_in_bank(line: String) -> i32 {
        let mut first = '0';
        let mut second = '0';

        for nbr in line.chars() {
            if first < nbr {
                first = nbr;
            } else if second < nbr && second < first {
                second = nbr;
            }
        }
        // bug volontaire conservé : on ne renvoie que le code ascii du premier
        return first as i32;
    }

    let mut count: i64 = 0;
    for bank in s.lines() {
        count += i64::from(found_best_nbr_in_bank(bank.to_string()));
    }
    count
}

// version 2 (partie 1) : on corrige le bug en formattant correctement les deux chiffres
// pour obtenir un vrai entier à deux chiffres avant addition.
#[allow(unused)]
pub fn d3p1_v2(s: &str) -> i64 {
    fn found_best_nbr_in_bank(line: String) -> i32 {
        let mut first = '0';
        let mut second = '0';

        for nbr in line.chars() {
            if first < nbr {
                first = nbr;
            } else if second < nbr && second < first {
                second = nbr;
            }
        }

        // on combine first et second en une vraie valeur numérique
        let final_nbr = format!("{}{}", first, second);
        let result: i32 = final_nbr.parse().unwrap();
        result
    }

    let mut count: i64 = 0;
    for bank in s.lines() {
        count += i64::from(found_best_nbr_in_bank(bank.to_string()));
    }
    count
}

// version 1 (partie 2) : approche brute force.
// on essaie toutes les fenêtres consécutives de 12 chiffres et on garde le maximum.
// problème : la consigne autorise des chiffres non consécutifs, donc cette version
// rate les meilleures combinaisons.
#[allow(unused)]
pub fn d3p2_v1(s: &str) -> i64 {
    fn found_best_nbr_in_bank(line: String) -> i64 {
        let mut max_joltage: i64 = 0;
        let chars: Vec<char> = line.chars().collect();

        // on vérifie qu'il y a au moins 12 chiffres
        if chars.len() < 12 {
            return 0;
        }

        for window in chars.windows(12) {
            // on transforme les 12 caracteres en une string
            let sequence: String = window.iter().collect();
            // on transforme tout ca en nombre
            let current_joltage: i64 = sequence.parse().unwrap_or(0);

            if current_joltage > max_joltage {
                max_joltage = current_joltage;
            }
        }

        max_joltage
    }

    let mut count: i64 = 0;
    for bank in s.lines() {
        count += found_best_nbr_in_bank(bank.to_string());
    }
    count
}

// version 2 (partie 2) : version finale gloutonne avec une pile.
// on parcourt les chiffres en gardant l'ordre original et on supprime les chiffres
// précédents tant qu'ils sont plus petits que celui qu'on regarde, dans la limite
// du nombre de chiffres qu'on peut se permettre de retirer (longueur - 12).
// à la fin on ne garde que les 12 premiers de la pile.
#[allow(unused)]
pub fn d3p2_v2(s: &str) -> i64 {
    fn found_best_nbr_in_bank(line: String) -> i64 {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() < 12 {
            return 0;
        }
        let to_remove = chars.len() - 12;
        let mut removed = 0;
        let mut stack: Vec<char> = Vec::new();

        for c in chars {
            // tant qu'on peut supprimer et que le chiffre actuel est plus grand que
            // le dernier ajoute à notre pile on retire le dernier
            while removed < to_remove && !stack.is_empty() && stack.last().unwrap() < &c {
                stack.pop();
                removed += 1;
            }
            stack.push(c);
        }

        // on s'assure de ne garder que les 12 premiers au cas où on n'a pas assez supprimé
        let result_str: String = stack.iter().take(12).collect();
        let max_joltage: i64 = result_str.parse().unwrap_or(0);

        max_joltage
    }

    let mut count: i64 = 0;
    for bank in s.lines() {
        count += found_best_nbr_in_bank(bank.to_string());
    }
    count
}

#[allow(unused)]
pub fn d3p1(s: &str) -> i64 {
    d3p1_v2(s)
}

#[allow(unused)]
pub fn d3p2(s: &str) -> i64 {
    d3p2_v2(s)
}

#[cfg(test)]
mod tests {
    use crate::d3::{d3p1, d3p2};

    #[test]
    fn d3p1_test() {
        let s = include_str!("d3_test.txt");
        let result: i64 = d3p1(s);
        println!("result: {}", result);
        // pour chaque ligne du fichier de test on prend "first|second" selon l'algorithme :
        // (la logique conserve volontairement le bug : second n'est jamais réévalué
        // quand first est remplacé, donc la somme dépend de l'ordre d'apparition)
        assert_eq!(371, result);
    }

    #[test]
    fn d3p2_test() {
        let s = include_str!("d3_test.txt");
        let result: i64 = d3p2(s);
        println!("result: {}", result);
        // somme des plus grands nombres à 12 chiffres extraits par la pile gloutonne
        assert_eq!(3_121_910_778_619, result);
    }
}
