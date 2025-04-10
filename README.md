# Système d'Élevage Pokémon en Rust

Un système de gestion d'élevage de Pokémon développé en Rust. Cette application permet de créer, gérer, entraîner et reproduire des Pokémon en utilisant les données de l'API PokéAPI.

## Fonctionnalités

- **Gestion des Pokémon :** Création, affichage et suppression de Pokémon
- **Types de Pokémon :** Prise en charge de 16 types différents (Feu, Eau, Plante, etc.)
- **Système d'expérience :** Les Pokémon gagnent des niveaux après avoir accumulé suffisamment d'XP
- **Reproduction :** Possibilité de créer de nouveaux Pokémon par reproduction
- **API Intégration :** Récupération de données depuis PokéAPI pour accéder aux 151 premiers Pokémon
- **Persistance des données :** Sauvegarde et chargement de l'élevage à partir d'un fichier
- **Triage :** Organisation des Pokémon par niveau ou par type

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (édition 2021 ou supérieure)
- Accès à Internet pour l'API PokéAPI

## Installation

1. Clonez ce dépôt :

```bash
git clone https://github.com/votre-nom/elevage-pokemon.git
cd elevage-pokemon-rust
```

2. Ajoutez les dépendances nécessaires à votre fichier `Cargo.toml` :

```toml
[dependencies]
rand = "0.8.5"
reqwest = { version = "0.11", features = ["json", "blocking"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

3. Compilez et exécutez le projet :

```bash
cargo build
cargo run
```

## Utilisation

Une fois lancée, l'application présente un menu interactif avec plusieurs options :

### Menu principal

```
===== GESTION D'ÉLEVAGE POKEMON =====
1. Ajouter un Pokémon
2. Ajouter un Pokémon aléatoire depuis l'API
3. Afficher tous les Pokémon
4. Entraîner tous les Pokémon
5. Tenter une reproduction
6. Trier les Pokémon par niveau
7. Trier les Pokémon par type
8. Sauvegarder l'élevage
9. Charger un élevage
10. Afficher la liste des 151 premiers Pokémon
11. Quitter
```

### Ajouter un Pokémon

Permet de créer un nouveau Pokémon en spécifiant :

- Son nom
- Son type (parmi les 16 disponibles)
- Son genre (Mâle ou Femelle)

### Ajouter un Pokémon aléatoire

Génère automatiquement un Pokémon aléatoire parmi les 151 premiers de la base de données PokéAPI.

### Afficher tous les Pokémon

Liste tous les Pokémon de l'élevage avec leurs caractéristiques :

- Nom
- Type
- Niveau
- Expérience
- Genre

### Entraîner les Pokémon

Permet d'augmenter l'expérience de tous les Pokémon. Pour chaque tranche de 100 XP, un Pokémon gagne un niveau.

### Reproduction

Permet de tenter la reproduction entre deux Pokémon compatibles. Conditions de compatibilité :

- Même type
- Genres opposés (un mâle et une femelle)
- Niveau minimum de 10 pour les deux Pokémon

Si la reproduction réussit, un nouveau Pokémon de niveau 1 est ajouté à l'élevage.

### Trier les Pokémon

Organisation des Pokémon de l'élevage par niveau (décroissant) ou par type.

### Sauvegarder/Charger

Permet de sauvegarder l'état actuel de l'élevage dans un fichier ou de charger un élevage précédemment sauvegardé.

### Liste des Pokémon

Affiche la liste complète des 151 premiers Pokémon disponibles via l'API.

## Structure du code

Le projet est organisé autour de plusieurs structures clés :

### `TypePokemon`

Une énumération représentant les différents types de Pokémon disponibles.

### `Genre`

Une énumération pour représenter le genre des Pokémon (Mâle ou Femelle).

### `Pokemon`

Structure principale contenant toutes les informations sur un Pokémon :

- nom
- niveau
- type_pokemon
- experience
- genre

### `Elevage`

Structure qui gère l'ensemble des Pokémon et fournit des méthodes pour :

- Ajouter, afficher et gérer les Pokémon
- Entraîner les Pokémon
- Gérer la reproduction
- Sauvegarder et charger les données

## Exemple de flux de travail

1. Démarrez l'application
2. Ajoutez quelques Pokémon (manuellement ou via l'API)
3. Entraînez vos Pokémon pour augmenter leur niveau
4. Tentez une reproduction entre Pokémon compatibles
5. Triez vos Pokémon par niveau pour voir les plus puissants
6. Sauvegardez votre élevage pour y revenir plus tard

## Fonctionnalités de l'API

L'application utilise l'API PokéAPI pour récupérer des informations sur les Pokémon :

- Noms des Pokémon
- Types des Pokémon
- Liste des 151 premiers Pokémon

## Licence

[Insérez votre licence ici]

---

Développé dans le cadre d'un TP d'apprentissage de Rust.
