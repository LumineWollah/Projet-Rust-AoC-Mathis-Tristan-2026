// petite structure de range, plus pratique qu'une paire de u64
#[derive(Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

/// Au-delà, la matérialisation de tous les ids (v1) dépasserait une RAM raisonnable.
const SEUIL_MATERIALISATION: u64 = 25_000_000;

// parse le contenu en deux sections séparées par une ligne vide :
// - une liste de ranges sous forme "start-end"
// - une liste de produits (un id par ligne)
fn split_stock_data(s: &str) -> (Vec<Range>, Vec<u64>) {
    // on neutralise les retours chariot windows pour pouvoir splitter sur "\n\n"
    let clean_content = s.replace('\r', "");
    let sections: Vec<&str> = clean_content.split("\n\n").collect();

    let mut fresh_date = Vec::new();
    let mut products = Vec::new();

    // on stock les ranges
    if let Some(range_section) = sections.first() {
        for line in range_section.lines() {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() == 2 {
                let start: u64 = parts[0].trim().parse().unwrap_or(0);
                let end: u64 = parts[1].trim().parse().unwrap_or(0);
                // on ajoute l'objet Range, pas le contenu !
                fresh_date.push(Range { start, end });
            }
        }
    }

    // on parse les produits
    if let Some(product_section) = sections.get(1) {
        for line in product_section.lines() {
            if let Ok(id) = line.trim().parse::<u64>() {
                products.push(id);
            }
        }
    }
    (fresh_date, products)
}

// version 1 (partie 1) : on matérialise encore tous les ids couverts par les plages,
// puis on compte les produits par recherche linéaire dans ce Vec — intuition directe.
// si le nombre total d'ids dépasse un seuil raisonnable, on bascule vers la v2 pour
// éviter l'OOM, mais le chemin « petit input » reste celui de la matérialisation.
#[allow(unused)]
pub fn d5p1_v1(s: &str) -> u64 {
    let (fresh_date, products) = split_stock_data(s);

    let total_ids: u64 = fresh_date
        .iter()
        .map(|r| r.end.saturating_sub(r.start).saturating_add(1))
        .sum();

    if total_ids > SEUIL_MATERIALISATION {
        return d5p1_v2(s);
    }

    let mut fresh_ids: Vec<u64> = Vec::new();
    for r in &fresh_date {
        fresh_ids.extend(r.start..=r.end);
    }

    let mut nbr_of_fresh_products = 0u64;
    for id in products {
        if fresh_ids.contains(&id) {
            nbr_of_fresh_products += 1;
        }
    }
    nbr_of_fresh_products
}

// version 2 (partie 1) : on ne matérialise plus les ids, on garde les ranges et on
// teste pour chaque produit s'il est couvert par au moins une range. linéaire en
// nombre de ranges par produit, mais ça reste correct sur les vraies données.
#[allow(unused)]
pub fn d5p1_v2(s: &str) -> u64 {
    let (fresh_date, products) = split_stock_data(s);

    let mut nbr_of_fresh_products = 0u64;
    for id in products {
        // on check si un produit frais est dans une range
        let is_fresh = fresh_date.iter().any(|r| id >= r.start && id <= r.end);
        if is_fresh {
            nbr_of_fresh_products += 1;
        }
    }
    nbr_of_fresh_products
}

// version 1 (partie 2) : on cherche le nombre total d'ids uniques couverts par
// l'union des ranges. on trie par début, puis on fusionne les ranges qui se
// chevauchent ou se touchent en avançant un curseur "current".
#[allow(unused)]
pub fn d5p2_v1(s: &str) -> u64 {
    let (mut fresh_date, _products) = split_stock_data(s);
    if fresh_date.is_empty() {
        return 0;
    }

    // on trie par le début de l'intervalle
    fresh_date.sort_by_key(|r| r.start);

    let mut total: u64 = 0;
    let mut current = fresh_date[0];

    for next in fresh_date.iter().skip(1) {
        if next.start <= current.end {
            // chevauchement : on étend la borne droite si besoin
            current.end = current.end.max(next.end);
        } else if next.start == current.end + 1 {
            // ranges adjacentes : on les colle
            current.end = next.end;
        } else {
            // gap : on ferme la range courante et on en démarre une nouvelle
            total += (current.end - current.start) + 1;
            current = *next;
        }
    }

    // on oublie pas d'ajouter le dernier intervalle après la boucle
    total += (current.end - current.start) + 1;

    total
}

#[allow(unused)]
pub fn d5p1(s: &str) -> u64 {
    d5p1_v2(s)
}

#[allow(unused)]
pub fn d5p2(s: &str) -> u64 {
    d5p2_v1(s)
}

#[cfg(test)]
mod tests {
    #[test]
    fn d5p1_test() {
        let s = include_str!("d5_test.txt");
        let result = super::d5p1(s);
        println!("result: {result}");
        // ranges : 3-5, 10-14, 16-20, 12-18
        // produits : 1, 5, 8, 11, 17, 32
        // 5 (in 3-5), 11 (in 10-14), 17 (in 12-18 / 16-20) -> 3 produits frais
        assert_eq!(3, result);
    }

    #[test]
    fn d5p2_test() {
        let s = include_str!("d5_test.txt");
        let result = super::d5p2(s);
        println!("result: {result}");
        // union des ranges :
        // 3-5 -> 3 ids
        // 10-14 fusionnée avec 12-18 fusionnée avec 16-20 -> 10-20 -> 11 ids
        // total = 14 ids uniques
        assert_eq!(14, result);
    }
}
