use rand::Rng;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};

// Partie 1: Définir les Pokémon
#[derive(Debug, Clone, PartialEq)]
enum TypePokemon {
    Feu,
    Eau,
    Plante,
    Electrik,
    Normal,
    Psy,
    Poison,
    Sol,
    Vol,
    Combat,
    Roche,
    Insecte,
    Spectre,
    Glace,
    Dragon,
    Fee,
}

impl fmt::Display for TypePokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypePokemon::Feu => write!(f, "Feu"),
            TypePokemon::Eau => write!(f, "Eau"),
            TypePokemon::Plante => write!(f, "Plante"),
            TypePokemon::Electrik => write!(f, "Electrik"),
            TypePokemon::Normal => write!(f, "Normal"),
            TypePokemon::Psy => write!(f, "Psy"),
            TypePokemon::Poison => write!(f, "Poison"),
            TypePokemon::Sol => write!(f, "Sol"),
            TypePokemon::Vol => write!(f, "Vol"),
            TypePokemon::Combat => write!(f, "Combat"),
            TypePokemon::Roche => write!(f, "Roche"),
            TypePokemon::Insecte => write!(f, "Insecte"),
            TypePokemon::Spectre => write!(f, "Spectre"),
            TypePokemon::Glace => write!(f, "Glace"),
            TypePokemon::Dragon => write!(f, "Dragon"),
            TypePokemon::Fee => write!(f, "Fée"),
        }
    }
}

// Conversion des types de la PokeAPI vers notre énumération
fn convert_type_from_api(api_type: &str) -> TypePokemon {
    match api_type.to_lowercase().as_str() {
        "fire" => TypePokemon::Feu,
        "water" => TypePokemon::Eau,
        "grass" => TypePokemon::Plante,
        "electric" => TypePokemon::Electrik,
        "normal" => TypePokemon::Normal,
        "psychic" => TypePokemon::Psy,
        "poison" => TypePokemon::Poison,
        "ground" => TypePokemon::Sol,
        "flying" => TypePokemon::Vol,
        "fighting" => TypePokemon::Combat,
        "rock" => TypePokemon::Roche,
        "bug" => TypePokemon::Insecte,
        "ghost" => TypePokemon::Spectre,
        "ice" => TypePokemon::Glace,
        "dragon" => TypePokemon::Dragon,
        "fairy" => TypePokemon::Fee,
        _ => TypePokemon::Normal, // Par défaut
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Genre {
    Male,
    Femelle,
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Genre::Male => write!(f, "Mâle"),
            Genre::Femelle => write!(f, "Femelle"),
        }
    }
}

#[derive(Debug, Clone)]
struct Pokemon {
    nom: String,
    niveau: u32,
    type_pokemon: TypePokemon,
    experience: u32,
    genre: Genre,
}

// Structures pour désérialiser la réponse de l'API
#[derive(Deserialize, Debug)]
struct PokemonApiResponse {
    name: String,
    types: Vec<TypeWrapper>,
}

#[derive(Deserialize, Debug)]
struct TypeWrapper {
    #[serde(rename = "type")]
    type_info: TypeInfo,
}

#[derive(Deserialize, Debug)]
struct TypeInfo {
    name: String,
}

#[derive(Deserialize, Debug)]
struct PokemonListResponse {
    results: Vec<PokemonListItem>,
}

#[derive(Deserialize, Debug)]
struct PokemonListItem {
    name: String,
    url: String,
}

// Partie 2: Fonctions et comportements
impl Pokemon {
    // Constructeur pour un nouveau Pokémon
    fn new(nom: String, type_pokemon: TypePokemon, genre: Genre) -> Self {
        Pokemon {
            nom,
            niveau: 1,
            type_pokemon,
            experience: 0,
            genre,
        }
    }

    // Récupérer un Pokémon depuis l'API
    fn from_api(pokemon_id: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new();
        let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id);

        let response = client.get(&url).send()?;
        let pokemon_data: PokemonApiResponse = response.json()?;

        // Obtenir le premier type du Pokémon
        let pokemon_type = if !pokemon_data.types.is_empty() {
            convert_type_from_api(&pokemon_data.types[0].type_info.name)
        } else {
            TypePokemon::Normal
        };

        // Générer un genre aléatoire
        let mut rng = rand::thread_rng();
        let genre = if rng.gen_bool(0.5) {
            Genre::Male
        } else {
            Genre::Femelle
        };

        // Première lettre en majuscule pour le nom
        let nom = pokemon_data
            .name
            .chars()
            .next()
            .unwrap_or('p')
            .to_uppercase()
            .collect::<String>()
            + &pokemon_data.name[1..];

        Ok(Pokemon::new(nom, pokemon_type, genre))
    }

    // Génère un Pokémon aléatoire parmi les 151 premiers
    fn generer_aleatoire(nom: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        let pokemon_id = rng.gen_range(1..=151);

        // Si un nom est fourni, on l'utilise, sinon on prend celui de l'API
        let pokemon = Self::from_api(pokemon_id)?;

        if let Some(custom_name) = nom {
            Ok(Pokemon {
                nom: custom_name,
                ..pokemon
            })
        } else {
            Ok(pokemon)
        }
    }

    // Gagner de l'expérience
    fn gagner_xp(&mut self, points: u32) {
        self.experience += points;

        // Vérifier si le Pokémon peut monter de niveau
        let niveaux_gagnes = self.experience / 100;
        if niveaux_gagnes > 0 {
            self.niveau += niveaux_gagnes;
            self.experience %= 100;
            println!("{} monte au niveau {}!", self.nom, self.niveau);
        }
    }

    // Afficher les informations du Pokémon
    fn afficher(&self) {
        println!("Nom: {}", self.nom);
        println!("Type: {}", self.type_pokemon);
        println!("Niveau: {}", self.niveau);
        println!("XP: {}/100", self.experience);
        println!("Genre: {}", self.genre);
        println!("------------------------");
    }

    // Vérifier si ce Pokémon peut se reproduire avec un autre
    fn peut_se_reproduire_avec(&self, autre: &Pokemon) -> bool {
        // Conditions: même type, genres opposés, niveau suffisant (au moins 10)
        self.type_pokemon == autre.type_pokemon
            && self.genre != autre.genre
            && self.niveau >= 10
            && autre.niveau >= 10
    }
}

// Liste des 151 premiers Pokémon
fn recuperer_liste_pokemon() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://pokeapi.co/api/v2/pokemon?limit=151";

    let response = client.get(url).send()?;
    let pokemon_list: PokemonListResponse = response.json()?;

    let noms = pokemon_list
        .results
        .into_iter()
        .map(|item| {
            // Première lettre en majuscule
            let nom = item
                .name
                .chars()
                .next()
                .unwrap_or('p')
                .to_uppercase()
                .collect::<String>()
                + &item.name[1..];
            nom
        })
        .collect();

    Ok(noms)
}

// Partie 3: Fonction de reproduction
fn reproduction(pokemon1: &Pokemon, pokemon2: &Pokemon) -> Option<Pokemon> {
    if pokemon1.peut_se_reproduire_avec(pokemon2) {
        let mut rng = rand::thread_rng();

        // Génération du genre aléatoirement
        let genre = if rng.gen_bool(0.5) {
            Genre::Male
        } else {
            Genre::Femelle
        };

        // Possibilité de donner un nom aléatoire au lieu de "Mystère"
        let nom = if rng.gen_bool(0.7) {
            "Mystère".to_string()
        } else {
            let noms = [
                "Bébé", "Junior", "Mini", "Petit", "Toudou", "Poupon", "Mignon", "Doux",
            ];
            noms[rng.gen_range(0..noms.len())].to_string()
        };

        // Nouveau Pokémon hérite du type des parents
        Some(Pokemon {
            nom,
            niveau: 1,
            type_pokemon: pokemon1.type_pokemon.clone(),
            experience: 0,
            genre,
        })
    } else {
        None
    }
}

// Partie 4: Gestion de l'élevage
struct Elevage {
    pokemon: Vec<Pokemon>,
}

impl Elevage {
    // Créer un nouvel élevage
    fn new() -> Self {
        Elevage {
            pokemon: Vec::new(),
        }
    }

    // Ajouter un Pokémon à l'élevage
    fn ajouter_pokemon(&mut self, pokemon: Pokemon) {
        println!("{} a été ajouté à l'élevage!", pokemon.nom);
        self.pokemon.push(pokemon);
    }

    // Afficher tous les Pokémon de l'élevage
    fn afficher_tous_pokemon(&self) {
        if self.pokemon.is_empty() {
            println!("L'élevage est vide!");
            return;
        }

        println!("\n===== LISTE DES POKEMON =====");
        for (i, pokemon) in self.pokemon.iter().enumerate() {
            println!("Pokemon #{}", i + 1);
            pokemon.afficher();
        }
    }

    // Entraîner tous les Pokémon (gain d'XP)
    fn entrainer_tous_pokemon(&mut self, xp: u32) {
        if self.pokemon.is_empty() {
            println!("Aucun Pokémon à entraîner!");
            return;
        }

        println!("Entraînement de tous les Pokémon...");
        for pokemon in &mut self.pokemon {
            pokemon.gagner_xp(xp);
        }
        println!("Entraînement terminé!");
    }

    // Tenter une reproduction entre deux Pokémon
    fn tenter_reproduction(&mut self, index1: usize, index2: usize) -> bool {
        // Vérifier que les indices sont valides
        if index1 >= self.pokemon.len() || index2 >= self.pokemon.len() {
            println!("Indices invalides!");
            return false;
        }

        // Cloner les Pokémon pour éviter les problèmes d'emprunt
        let pokemon1 = self.pokemon[index1].clone();
        let pokemon2 = self.pokemon[index2].clone();

        println!(
            "Tentative de reproduction entre {} et {}...",
            pokemon1.nom, pokemon2.nom
        );

        // Tenter la reproduction
        if let Some(bebe) = reproduction(&pokemon1, &pokemon2) {
            println!("Félicitations! Un nouveau Pokémon est né: {}!", bebe.nom);
            self.pokemon.push(bebe);
            true
        } else {
            println!("La reproduction a échoué. Les Pokémon ne sont pas compatibles.");
            false
        }
    }

    // Bonus: Trier les Pokémon par niveau
    fn trier_par_niveau(&mut self) {
        self.pokemon.sort_by(|a, b| b.niveau.cmp(&a.niveau));
        println!("Pokémon triés par niveau (décroissant)!");
    }

    // Bonus: Trier les Pokémon par type
    fn trier_par_type(&mut self) {
        self.pokemon.sort_by(|a, b| {
            let type_a = format!("{}", a.type_pokemon);
            let type_b = format!("{}", b.type_pokemon);
            type_a.cmp(&type_b)
        });
        println!("Pokémon triés par type!");
    }

    // Bonus: Sauvegarder l'élevage dans un fichier
    fn sauvegarder(&self, fichier: &str) -> Result<(), io::Error> {
        let mut file = File::create(fichier)?;

        for pokemon in &self.pokemon {
            writeln!(
                file,
                "{}|{}|{}|{}|{}",
                pokemon.nom,
                pokemon.niveau,
                format!("{}", pokemon.type_pokemon),
                pokemon.experience,
                format!("{}", pokemon.genre)
            )?;
        }

        println!("Élevage sauvegardé dans '{}'!", fichier);
        Ok(())
    }

    // Bonus: Charger l'élevage depuis un fichier
    fn charger(fichier: &str) -> Result<Self, io::Error> {
        let mut elevage = Elevage::new();
        let mut file = File::open(fichier)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        for line in contents.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 5 {
                // Convertir les données
                let nom = parts[0].to_string();
                let niveau = parts[1].parse::<u32>().unwrap_or(1);

                // Convertir le type
                let type_pokemon = match parts[2] {
                    "Feu" => TypePokemon::Feu,
                    "Eau" => TypePokemon::Eau,
                    "Plante" => TypePokemon::Plante,
                    "Electrik" => TypePokemon::Electrik,
                    "Normal" => TypePokemon::Normal,
                    "Psy" => TypePokemon::Psy,
                    "Poison" => TypePokemon::Poison,
                    "Sol" => TypePokemon::Sol,
                    "Vol" => TypePokemon::Vol,
                    "Combat" => TypePokemon::Combat,
                    "Roche" => TypePokemon::Roche,
                    "Insecte" => TypePokemon::Insecte,
                    "Spectre" => TypePokemon::Spectre,
                    "Glace" => TypePokemon::Glace,
                    "Dragon" => TypePokemon::Dragon,
                    "Fée" => TypePokemon::Fee,
                    _ => TypePokemon::Normal, // Par défaut
                };

                let experience = parts[3].parse::<u32>().unwrap_or(0);

                // Convertir le genre
                let genre = match parts[4] {
                    "Mâle" => Genre::Male,
                    "Femelle" => Genre::Femelle,
                    _ => Genre::Male, // Par défaut
                };

                // Créer le Pokémon et l'ajouter à l'élevage
                let pokemon = Pokemon {
                    nom,
                    niveau,
                    type_pokemon,
                    experience,
                    genre,
                };

                elevage.pokemon.push(pokemon);
            }
        }

        println!("Élevage chargé depuis '{}'!", fichier);
        Ok(elevage)
    }
}

fn lire_saisie(message: &str) -> String {
    println!("{}", message);
    let mut saisie = String::new();
    io::stdin()
        .read_line(&mut saisie)
        .expect("Erreur lors de la lecture de la saisie");
    saisie.trim().to_string()
}

fn lire_nombre(message: &str) -> usize {
    loop {
        let saisie = lire_saisie(message);
        match saisie.parse::<usize>() {
            Ok(nombre) => return nombre,
            Err(_) => println!("Veuillez entrer un nombre valide."),
        }
    }
}

fn afficher_menu() {
    println!("\n===== GESTION D'ÉLEVAGE POKEMON =====");
    println!("1. Ajouter un Pokémon");
    println!("2. Ajouter un Pokémon aléatoire depuis l'API");
    println!("3. Afficher tous les Pokémon");
    println!("4. Entraîner tous les Pokémon");
    println!("5. Tenter une reproduction");
    println!("6. Trier les Pokémon par niveau");
    println!("7. Trier les Pokémon par type");
    println!("8. Sauvegarder l'élevage");
    println!("9. Charger un élevage");
    println!("10. Afficher la liste des 151 premiers Pokémon");
    println!("11. Quitter");
    print!("Votre choix: ");
    io::stdout().flush().unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut elevage = Elevage::new();

    // Ajouter quelques Pokémon de départ (depuis l'API)
    println!("Chargement des Pokémon initiaux depuis l'API...");
    if let Ok(pikachu) = Pokemon::from_api(25) {
        let mut pikachu_modifie = pikachu;
        pikachu_modifie.niveau = 15;
        elevage.ajouter_pokemon(pikachu_modifie);
    }

    if let Ok(bulbizarre) = Pokemon::from_api(1) {
        let mut bulbizarre_modifie = bulbizarre;
        bulbizarre_modifie.niveau = 12;
        bulbizarre_modifie.genre = Genre::Femelle;
        elevage.ajouter_pokemon(bulbizarre_modifie);
    }

    if let Ok(salameche) = Pokemon::from_api(4) {
        let mut salameche_modifie = salameche;
        salameche_modifie.niveau = 14;
        elevage.ajouter_pokemon(salameche_modifie);
    }

    if let Ok(carapuce) = Pokemon::from_api(7) {
        let mut carapuce_modifie = carapuce;
        carapuce_modifie.niveau = 11;
        carapuce_modifie.genre = Genre::Femelle;
        elevage.ajouter_pokemon(carapuce_modifie);
    }

    loop {
        afficher_menu();

        let choix = lire_nombre("");

        match choix {
            1 => {
                // Ajouter un Pokémon
                let nom = lire_saisie("Entrez le nom du Pokémon:");

                println!("Types disponibles:");
                println!("1. Feu");
                println!("2. Eau");
                println!("3. Plante");
                println!("4. Electrik");
                println!("5. Normal");
                println!("6. Psy");
                println!("7. Poison");
                println!("8. Sol");
                println!("9. Vol");
                println!("10. Combat");
                println!("11. Roche");
                println!("12. Insecte");
                println!("13. Spectre");
                println!("14. Glace");
                println!("15. Dragon");
                println!("16. Fée");
                let type_choix = lire_nombre("Choisissez le type (1-16):");

                let type_pokemon = match type_choix {
                    1 => TypePokemon::Feu,
                    2 => TypePokemon::Eau,
                    3 => TypePokemon::Plante,
                    4 => TypePokemon::Electrik,
                    5 => TypePokemon::Normal,
                    6 => TypePokemon::Psy,
                    7 => TypePokemon::Poison,
                    8 => TypePokemon::Sol,
                    9 => TypePokemon::Vol,
                    10 => TypePokemon::Combat,
                    11 => TypePokemon::Roche,
                    12 => TypePokemon::Insecte,
                    13 => TypePokemon::Spectre,
                    14 => TypePokemon::Glace,
                    15 => TypePokemon::Dragon,
                    16 => TypePokemon::Fee,
                    _ => {
                        println!("Type invalide, Normal par défaut.");
                        TypePokemon::Normal
                    }
                };

                println!("Genres disponibles:");
                println!("1. Mâle");
                println!("2. Femelle");
                let genre_choix = lire_nombre("Choisissez le genre (1-2):");

                let genre = match genre_choix {
                    1 => Genre::Male,
                    2 => Genre::Femelle,
                    _ => {
                        println!("Genre invalide, Mâle par défaut.");
                        Genre::Male
                    }
                };

                let pokemon = Pokemon::new(nom, type_pokemon, genre);
                elevage.ajouter_pokemon(pokemon);
            }
            2 => {
                // Ajouter un Pokémon aléatoire depuis l'API
                let choix_nom = lire_saisie("Voulez-vous donner un nom au Pokémon? (O/N):");
                if choix_nom.to_uppercase() == "O" {
                    let nom = lire_saisie("Entrez le nom du Pokémon:");
                    match Pokemon::generer_aleatoire(Some(nom)) {
                        Ok(pokemon) => elevage.ajouter_pokemon(pokemon),
                        Err(e) => println!("Erreur lors de la récupération du Pokémon: {}", e),
                    }
                } else {
                    match Pokemon::generer_aleatoire(None) {
                        Ok(pokemon) => elevage.ajouter_pokemon(pokemon),
                        Err(e) => println!("Erreur lors de la récupération du Pokémon: {}", e),
                    }
                }
            }
            3 => {
                // Afficher tous les Pokémon
                elevage.afficher_tous_pokemon();
            }
            4 => {
                // Entraîner tous les Pokémon
                let xp = lire_nombre("Combien d'XP donner à chaque Pokémon?:");
                elevage.entrainer_tous_pokemon(xp as u32);
            }
            5 => {
                // Tenter une reproduction
                elevage.afficher_tous_pokemon();
                if elevage.pokemon.len() >= 2 {
                    let index1 = lire_nombre("Choisissez le premier Pokémon (numéro):") - 1;
                    let index2 = lire_nombre("Choisissez le second Pokémon (numéro):") - 1;
                    elevage.tenter_reproduction(index1, index2);
                } else {
                    println!("Il faut au moins 2 Pokémon pour tenter une reproduction!");
                }
            }
            6 => {
                // Trier par niveau
                elevage.trier_par_niveau();
                elevage.afficher_tous_pokemon();
            }
            7 => {
                // Trier par type
                elevage.trier_par_type();
                elevage.afficher_tous_pokemon();
            }
            8 => {
                // Sauvegarder l'élevage
                let fichier = lire_saisie("Nom du fichier de sauvegarde:");
                if let Err(e) = elevage.sauvegarder(&fichier) {
                    println!("Erreur lors de la sauvegarde: {}", e);
                }
            }
            9 => {
                // Charger un élevage
                let fichier = lire_saisie("Nom du fichier à charger:");
                match Elevage::charger(&fichier) {
                    Ok(nouvel_elevage) => {
                        elevage = nouvel_elevage;
                    }
                    Err(e) => {
                        println!("Erreur lors du chargement: {}", e);
                    }
                }
            }
            10 => {
                // Afficher la liste des 151 premiers Pokémon
                println!("Récupération de la liste des 151 premiers Pokémon...");
                match recuperer_liste_pokemon() {
                    Ok(pokemons) => {
                        println!("\n===== LISTE DES 151 PREMIERS POKEMON =====");
                        for (i, nom) in pokemons.iter().enumerate() {
                            println!("#{}: {}", i + 1, nom);
                        }
                    }
                    Err(e) => println!("Erreur lors de la récupération des Pokémon: {}", e),
                }
            }
            11 => {
                println!("Au revoir!");
                break;
            }
            _ => println!("Option invalide. Veuillez réessayer."),
        }
    }

    Ok(())
}
