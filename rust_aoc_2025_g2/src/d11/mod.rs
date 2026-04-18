use std::collections::HashMap;

// helper commun : on parse un réseau d'appareils sous la forme :
//     nom: sortie1 sortie2 sortie3
// en un HashMap<String, Vec<String>>.
fn parse_reseau(s: &str) -> HashMap<String, Vec<String>> {
    let mut reseau = HashMap::new();
    for ligne in s.lines() {
        if ligne.is_empty() {
            continue;
        }
        let parties: Vec<&str> = ligne.split(": ").collect();
        if parties.len() != 2 {
            continue;
        }
        let nom = parties[0].trim();
        let sorties: Vec<String> = parties[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        reseau.insert(nom.to_string(), sorties);
    }
    reseau
}

// version 1 (partie 1) : on compte tous les chemins partant de "you" et
// arrivant à "out". récursion DFS toute simple, sans contrainte de passage.
// (le nom "you" est spécifique à mon input et au fichier de test, c'est
// hardcodé volontairement parce que c'était le départ donné dans la consigne.)
fn compter_chemins(actuel: &str, reseau: &HashMap<String, Vec<String>>) -> i64 {
    // si on arrive a out on a trouve un chemin
    if actuel == "out" {
        return 1;
    }

    // si l appareil n existe pas dans notre liste on s arrete
    if !reseau.contains_key(actuel) {
        return 0;
    }

    let mut somme = 0;
    // on regarde chaque sortie de l appareil actuel
    if let Some(sorties) = reseau.get(actuel) {
        for suivante in sorties {
            // on continue le chemin a partir de la suivante
            somme += compter_chemins(suivante, reseau);
        }
    }
    somme
}

#[allow(unused)]
pub fn d11p1_v1(s: &str) -> i64 {
    let reseau = parse_reseau(s);
    // on lance la recherche a partir de you
    compter_chemins("you", &reseau)
}

// version 1 (partie 2) : maintenant on doit compter les chemins "svr -> out"
// qui passent obligatoirement par "dac" ET par "fft" en chemin.
// on transmet deux booléens "déjà vu dac" et "déjà vu fft" tout au long de la
// recursion. pas de cache donc ça explose en complexité sur les vrais inputs.
fn compter_chemins_speciaux(
    actuel: &str,
    mut a_vu_dac: bool,
    mut a_vu_fft: bool,
    reseau: &HashMap<String, Vec<String>>,
) -> i64 {
    // si on passe sur dac ou fft on met a jour nos badges
    if actuel == "dac" {
        a_vu_dac = true;
    }
    if actuel == "fft" {
        a_vu_fft = true;
    }

    // si on arrive a out
    if actuel == "out" {
        // le chemin n est valide que si on a vu les deux
        if a_vu_dac && a_vu_fft {
            return 1;
        } else {
            return 0;
        }
    }

    if !reseau.contains_key(actuel) {
        return 0;
    }

    let mut somme = 0;
    if let Some(sorties) = reseau.get(actuel) {
        for suivante in sorties {
            // on transmet l etat actuel aux suivants
            somme += compter_chemins_speciaux(suivante, a_vu_dac, a_vu_fft, reseau);
        }
    }
    somme
}

#[allow(unused)]
pub fn d11p2_v1(s: &str) -> i64 {
    let reseau = parse_reseau(s);
    // on lance a partir de svr avec dac=faux et fft=faux
    compter_chemins_speciaux("svr", false, false, &reseau)
}

// version 2 (partie 2) : on rajoute un cache (memoization).
// la clé est (nom_du_noeud, a_vu_dac, a_vu_fft) parce que le nombre de
// chemins restants vers "out" ne dépend que de cet état là. ça permet de
// repasser en quasi linéaire sur le nombre d'arêtes au lieu d'exponentiel.
fn compter_chemins_rapide(
    actuel: &str,
    mut dac: bool,
    mut fft: bool,
    reseau: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<(String, bool, bool), i64>,
) -> i64 {
    if actuel == "dac" {
        dac = true;
    }
    if actuel == "fft" {
        fft = true;
    }

    if actuel == "out" {
        return if dac && fft { 1 } else { 0 };
    }

    // si on a deja fait ce calcul precis, on rend le resultat direct
    let cle = (actuel.to_string(), dac, fft);
    if let Some(&res) = cache.get(&cle) {
        return res;
    }

    if !reseau.contains_key(actuel) {
        return 0;
    }

    let mut somme = 0;
    if let Some(sorties) = reseau.get(actuel) {
        for suivante in sorties {
            somme += compter_chemins_rapide(suivante, dac, fft, reseau, cache);
        }
    }

    // on enregistre le resultat dans le cache avant de partir
    cache.insert(cle, somme);
    somme
}

#[allow(unused)]
pub fn d11p2_v2(s: &str) -> i64 {
    let reseau = parse_reseau(s);
    let mut cache = HashMap::new();
    compter_chemins_rapide("svr", false, false, &reseau, &mut cache)
}

#[allow(unused)]
pub fn d11p1(s: &str) -> i64 {
    d11p1_v1(s)
}

#[allow(unused)]
pub fn d11p2(s: &str) -> i64 {
    d11p2_v2(s)
}

#[cfg(test)]
mod tests {
    use crate::d11::{d11p1, d11p2, d11p2_v1};

    #[test]
    fn d11p1_test() {
        let s = include_str!("d11_test.txt");
        let result = d11p1(s);
        println!("result: {}", result);
        // depuis "you" : you -> bbb|ccc, et différentes sorties vers "out".
        // chemins comptés :
        //   you->bbb->ddd->ggg->out
        //   you->bbb->eee->out
        //   you->ccc->ddd->ggg->out
        //   you->ccc->eee->out
        //   you->ccc->fff->out
        // soit 5 chemins
        assert_eq!(5, result);
    }

    // pour la partie 2 le fichier de test du dépôt n'a ni "svr", ni "dac",
    // ni "fft" ; on construit donc un mini réseau dédié inline qui contient
    // ces noms et qui permet de vérifier la logique de notre algorithme.
    #[allow(dead_code)]
    const D11P2_TEST: &str = "svr: dac fft a\n\
                              dac: out fft\n\
                              fft: out\n\
                              a: out\n";

    #[test]
    fn d11p2_test() {
        let result = d11p2(D11P2_TEST);
        println!("result: {}", result);
        // chemins valides (devant passer par dac ET par fft) :
        //   svr->dac->fft->out  (vu dac puis fft)
        //   svr->fft->? non : depuis fft on va à out direct, mais on n'a pas vu dac -> invalide
        //   svr->dac->out  invalide (pas vu fft)
        //   svr->a->out    invalide (ni dac ni fft)
        // -> 1 seul chemin valide
        assert_eq!(1, result);
    }

    #[test]
    fn d11p2_v1_meme_resultat_que_v2() {
        // sécurité : la version sans cache et la version avec cache doivent
        // donner exactement le même résultat sur ce mini réseau.
        let v1 = d11p2_v1(D11P2_TEST);
        let v2 = d11p2(D11P2_TEST);
        assert_eq!(v1, v2);
    }
}
