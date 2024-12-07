// Importation des bibliothèques nécessaires
use std::fs::File; // Pour travailler avec les fichiers
use std::io::{self, BufRead}; // Pour la lecture ligne par ligne et les opérations d'entrée/sortie
use std::path::Path; // Pour manipuler les chemins de fichiers
use std::collections::HashMap; // Pour gérer le comptage des occurrences

/// Fonction pour lire les données d'un fichier et retourner deux listes d'entiers
/// `P` est un type générique qui représente un chemin vers le fichier
fn read_data_from_file<P>(filename: P) -> io::Result<(Vec<i32>, Vec<i32>)>
where
    P: AsRef<Path>, // Cette contrainte signifie que `P` doit pouvoir être converti en un chemin de type `Path`
{
    // Ouvre le fichier en mode lecture
    let file = File::open(filename)?;
    // Utilise un buffer pour lire le fichier efficacement ligne par ligne
    let reader = io::BufReader::new(file);

    // Initialise deux vecteurs vides pour stocker les listes gauche et droite
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    // Parcourt chaque ligne du fichier
    for line in reader.lines() {
        // `line` est un résultat (`Result`) qui peut contenir une erreur, donc on utilise `if let Ok`
        if let Ok(record) = line {
            // Découpe la ligne en morceaux (séparés par des espaces), puis essaie de convertir chaque morceau en un entier
            let numbers: Vec<i32> = record
                .split_whitespace() // Sépare la ligne en "mots" en utilisant les espaces comme séparateurs
                .filter_map(|x| x.parse::<i32>().ok()) // Convertit chaque "mot" en entier et ignore ceux qui échouent
                .collect(); // Transforme l'itérateur en un vecteur

            // Si la ligne contient exactement deux nombres, on les ajoute aux listes
            if numbers.len() == 2 {
                left_list.push(numbers[0]); // Ajoute le premier nombre à la liste de gauche
                right_list.push(numbers[1]); // Ajoute le second nombre à la liste de droite
            }
        }
    }

    // Retourne les deux listes dans un tuple
    Ok((left_list, right_list))
}

/// Fonction pour calculer la distance totale entre deux listes triées
fn calculate_total_distance(mut left_list: Vec<i32>, mut right_list: Vec<i32>) -> i32 {
    // Trie les deux listes en place (ordre croissant)
    left_list.sort();
    right_list.sort();

    // Associe les éléments des deux listes dans l'ordre trié avec `zip`
    // Calcule la différence absolue entre chaque paire, et somme toutes les différences
    left_list
        .iter() // Crée un itérateur sur les éléments de `left_list`
        .zip(right_list.iter()) // Associe chaque élément de `left_list` avec l'élément correspondant de `right_list`
        .map(|(l, r)| (l - r).abs()) // Calcule la différence absolue entre les deux éléments
        .sum() // Calcule la somme des différences
}

/// Fonction pour calculer le score de similarité entre les deux listes
fn calculate_similarity_score(left_list: Vec<i32>, right_list: Vec<i32>) -> i32 {
    // Crée un dictionnaire (HashMap) pour compter les occurrences des éléments de la liste de droite
    let mut right_counts = HashMap::new();
    for &num in &right_list {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Calcule le score de similarité
    let mut similarity_score = 0;
    for &num in &left_list {
        if let Some(&count) = right_counts.get(&num) {
            similarity_score += num * count; // Ajoute le produit au score
        }
    }

    similarity_score
}

/// Fonction pour exécuter la première partie
fn part1() -> io::Result<()> {
    // Chemin du fichier contenant les données
    let path = "1.txt";
    // Appelle la fonction pour lire les données du fichier
    let (left_list, right_list) = read_data_from_file(path)?;

    // Appelle la fonction pour calculer la distance totale entre les listes
    let total_distance = calculate_total_distance(left_list, right_list);

    // Affiche la distance totale calculée
    println!("Distance totale : {}", total_distance);

    Ok(()) // Retour explicite que tout s'est bien passé
}

/// Fonction pour exécuter la deuxième partie
fn part2() -> io::Result<()> {
    // Chemin du fichier contenant les données
    let path = "1.txt";
    // Appelle la fonction pour lire les données du fichier
    let (left_list, right_list) = read_data_from_file(path)?;

    // Appelle la fonction pour calculer le score de similarité entre les deux listes
    let similarity_score = calculate_similarity_score(left_list, right_list);

    // Affiche le score de similarité calculé
    println!("Score de similarité : {}", similarity_score);

    Ok(()) // Retour explicite que tout s'est bien passé
}

/// Point d'entrée principal du programme
fn main() -> io::Result<()> {
    // Exécute la première partie
    part1()?;
    // Exécute la deuxième partie
    part2()?;

    Ok(()) // Retour explicite que tout s'est bien passé
}
