// version 1 (partie 1) : double boucle sur toutes les paires de positions i < j.
// idée naïve : on teste explicitement chaque couple (dizaine, unité) dans l'ordre
// de la ligne. correct mais O(n²) par banque.
#[allow(unused)]
pub fn d3p1_v1(s: &str) -> i64 {
    fn max_joltage_bank(line: &str) -> i64 {
        let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
        let n = digits.len();
        if n < 2 {
            return 0;
        }
        let mut best = 0i64;
        for i in 0..n {
            for j in (i + 1)..n {
                let v = i64::from(digits[i]) * 10 + i64::from(digits[j]);
                if v > best {
                    best = v;
                }
            }
        }
        best
    }

    let mut count: i64 = 0;
    for bank in s.lines() {
        if bank.trim().is_empty() {
            continue;
        }
        count += max_joltage_bank(bank);
    }
    count
}

// version 2 (partie 1) : une passe suffixe pour connaître le max à droite de chaque
// indice, puis un seul parcours O(n) par ligne. même résultat que la v1, moins de travail.
#[allow(unused)]
pub fn d3p1_v2(s: &str) -> i64 {
    fn max_joltage_bank(line: &str) -> i64 {
        let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
        let n = digits.len();
        if n < 2 {
            return 0;
        }
        // max_after[i] = meilleur chiffre strictement à droite de l'indice i
        let mut max_after = vec![0u8; n];
        for i in (0..n - 1).rev() {
            max_after[i] = digits[i + 1].max(max_after[i + 1]);
        }
        let mut best = 0i64;
        for i in 0..(n - 1) {
            let v = i64::from(digits[i]) * 10 + i64::from(max_after[i]);
            if v > best {
                best = v;
            }
        }
        best
    }

    let mut count: i64 = 0;
    for bank in s.lines() {
        if bank.trim().is_empty() {
            continue;
        }
        count += max_joltage_bank(bank);
    }
    count
}

// version 1 (partie 2) : programmation dynamique — on choisit exactement 12 chiffres
// dans l'ordre (sous-suite). encore une approche exhaustive dans l'esprit, mais avec
// mémo O(n·12) au lieu d'énumérer des combinaisons une par une à la main.
#[allow(unused)]
pub fn d3p2_v1(s: &str) -> i64 {
    fn max_joltage_bank(line: &str) -> i64 {
        let s: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
        let n = s.len();
        let k = 12;
        if n < k {
            return 0;
        }

        // dp[i][t] = meilleur nombre à t chiffres obtenu avec les caractères d'indice >= i
        let mut dp = vec![vec![None::<i64>; k + 1]; n + 1];
        for i in 0..=n {
            dp[i][0] = Some(0);
        }
        for t in 1..=k {
            dp[n][t] = None;
        }

        for i in (0..n).rev() {
            for t in 1..=k {
                let skip = dp[i + 1][t];
                let take = dp[i + 1][t - 1].map(|sub| {
                    i64::from(s[i]) * 10_i64.pow((t - 1) as u32) + sub
                });
                dp[i][t] = match (skip, take) {
                    (Some(a), Some(b)) => Some(a.max(b)),
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (None, None) => None,
                };
            }
        }

        dp[0][k].unwrap_or(0)
    }

    let mut count: i64 = 0;
    for bank in s.lines() {
        if bank.trim().is_empty() {
            continue;
        }
        count += max_joltage_bank(bank);
    }
    count
}

// version 2 (partie 2) : pile gloutonne — on retire les petits chiffres quand on peut
// encore « payer » des suppressions pour maximiser les 12 premiers de la pile finale.
#[allow(unused)]
pub fn d3p2_v2(s: &str) -> i64 {
    fn max_joltage_bank(line: String) -> i64 {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() < 12 {
            return 0;
        }
        let to_remove = chars.len() - 12;
        let mut removed = 0;
        let mut stack: Vec<char> = Vec::new();

        for c in chars {
            while removed < to_remove && !stack.is_empty() && stack.last().unwrap() < &c {
                stack.pop();
                removed += 1;
            }
            stack.push(c);
        }

        let result_str: String = stack.iter().take(12).collect();
        result_str.parse().unwrap_or(0)
    }

    let mut count: i64 = 0;
    for bank in s.lines() {
        if bank.trim().is_empty() {
            continue;
        }
        count += max_joltage_bank(bank.to_string());
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
        // exemple officiel AoC : 98 + 89 + 78 + 92
        assert_eq!(357, result);
    }

    #[test]
    fn d3p2_test() {
        let s = include_str!("d3_test.txt");
        let result: i64 = d3p2(s);
        println!("result: {}", result);
        assert_eq!(3_121_910_778_619, result);
    }
}
