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

impl Default for TrieNode {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }
    pub fn insert(&mut self, number: &str, name: &str) {
        let mut current_node = &mut self.root;
        
        for digit in number.chars() {
            // crée le nœud s'il n'existe pas, sinon retourne l'existant
            current_node = current_node
                .children
                .entry(digit)
                .or_insert_with(TrieNode::new);
        }
        current_node.is_end = true;
        current_node.contact_name = Some(name.to_string());
    }
    pub fn search(&self, number: &str) -> bool {
        let mut current_node = &self.root;

        for digit in number.chars() {
            match current_node.children.get(&digit) {
                Some(next) => current_node = next,
                None => return false,
            }
        }
        current_node.is_end
    }
    pub fn to_plantuml(&self) -> String {
        let mut output = String::new();
        output.push_str("@startmindmap\n");
        output.push_str("* Trie\n");
        self.root.generate_puml(&mut output, 2);
        output.push_str("@endmindmap\n");
        output
    }
}
