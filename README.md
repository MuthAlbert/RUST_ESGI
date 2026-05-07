# Gestionnaire de Numéros de Téléphone

> Implémentation d'un gestionnaire de numéros de téléphone basé sur une structure **Trie** (Prefix Tree), avec génération de visualisation via PlantUML.

---

## Auteurs

| Nom |
|-----|
| Albert |
| Bilal |
| Yunus |

---

## Description

Ce projet implémente un **Trie** (arbre préfixe) pour le stockage et l'accès efficace à des numéros de téléphone. Le Trie permet :

- Un **accès rapide** aux données
- Une **minimisation de l'espace de stockage** grâce à la suppression des doublons
- La **visualisation** de la structure sous forme de diagramme MindMap PlantUML

Le programme :
1. Lit un fichier JSON contenant les données utilisateurs
2. Construit le Trie en mémoire
3. Génère un fichier `.puml` pour visualiser la structure
