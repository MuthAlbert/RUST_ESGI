#![forbid(unsafe_code)]

use std::collections::HashMap;

pub struct TrieNode {
    // valeur = noeud enfant
    pub children: HashMap<char, TrieNode>,
    // true si ce nœud est la dernière lettre d'un numéro complet
    pub is_end: bool,
    // seulement si is_end == true
    pub contact_name: Option<String>,
}

impl TrieNode {
    // Crée un nœud vide
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
            contact_name: None,
        }
    }
    pub fn generate_puml(&self, output: &mut String, depth: usize) {
        // On trie les clés pour avoir un rendu organisé
        let mut keys: Vec<char> = self.children.keys().cloned().collect();
        keys.sort();

        for key in keys {
            let child = &self.children[&key];
            // Pour le niveau de profondeur
            let stars = "*".repeat(depth);

            if child.is_end {
                // On affiche le chiffre + le nom du contact
                let name = child
                    .contact_name
                    .as_deref()
                    .unwrap_or("inconnu");
                output.push_str(&format!("{} {} [{}]\n", stars, key, name));
            } else {
                // Noeud intermédiaire
                output.push_str(&format!("{} {}\n", stars, key));
            }
            // Appel récursif pour les enfants de cet enfant
            child.generate_puml(output, depth + 1);
        }
    }
}
