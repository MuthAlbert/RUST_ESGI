use std::fs;
use std::process;
use serde::Deserialize;
use projet_rust::Trie;

// Le JSON attendu 
#[derive(Deserialize, Debug)]
struct Contact {
    name: String,
    number: String,
}

// Lit le fichier JSON et retourne la liste des contacts
fn lire_contacts(json_path: &str) -> Vec<Contact> {
    println!("Lecture du fichier : {}", json_path);

    let json_content = fs::read_to_string(json_path).unwrap_or_else(|err| {
        eprintln!("Erreur : impossible de lire '{}' : {}", json_path, err);
        eprintln!("Vérifiez que le fichier existe dans le dossier 'data/'.");
        process::exit(1);
    });

    let contacts: Vec<Contact> = serde_json::from_str(&json_content).unwrap_or_else(|err| {
        eprintln!("Erreur : le fichier JSON est mal formaté : {}", err);
        process::exit(1);
    });

    println!("{} contacts chargés.", contacts.len());
    contacts
}

// Construit le Trie à partir d'une liste de contacts
fn construire_trie(contacts: &[Contact]) -> Trie {
    let mut trie = Trie::new();

    for contact in contacts {
        let clean_number: String = contact
            .number
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();

        if clean_number.is_empty() {
            eprintln!("Avertissement : numéro invalide ignoré pour '{}'", contact.name);
            continue;
        }

        trie.insert(&clean_number, &contact.name);
    }

    println!("Trie construit avec succès !");
    trie
}

// Génère le fichier PlantUML à partir du Trie
fn exporter_plantuml(trie: &Trie, output_path: &str) {}

fn main() {
    let contacts = lire_contacts("data/04_common_parts.json");
    let trie = construire_trie(&contacts);
    exporter_plantuml(&trie, "graph/04_common_parts.puml");
}
