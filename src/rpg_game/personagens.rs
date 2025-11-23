use std::{collections::HashMap, ptr::null};

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::rpg_game::itens::{Arma, Armadura, ItemTipo};

#[derive(Serialize, Deserialize)]
pub struct Personagem {
    pub id: Uuid,
    pub nome: String,
    pub vida: u32,
    pub forca:u32,
    pub nivel:u32,
    pub armadura: Armadura,
    pub defesa: u32,
    pub arma: Arma,
    pub classe: Classe,
    pub inventario: HashMap<Uuid, ItemTipo>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Classe {
    Guerreiro,
    Mago,
    Assassino,
}

impl Classe {
    pub fn as_str(&self) -> &str {
        match self {
            Classe::Guerreiro => "Guerreiro",
            Classe::Mago => "Mago",
            Classe::Assassino => "Assassino",
        }
    }
}

impl Personagem {
    pub fn new(nome: String, mut vida: u32, mut forca: u32, classe: Classe, armadura: Armadura, arma: Arma) -> Self {
        
        if vida == 0 && forca == 0 {
            match classe {
                Classe::Guerreiro => {
                    vida = 100;
                    forca = 15;
                }
                Classe::Mago => {
                    vida = 70;
                    forca = 20;
                }
                Classe::Assassino => {
                    vida = 60;
                    forca = 28;
                }
            }
        }

        Personagem {
            id: Uuid::new_v4(),
            nome,
            vida,
            forca,
            nivel: 1,
            armadura: armadura.clone(),
            defesa: armadura.get_defesa(),
            arma,
            classe,
            inventario: HashMap::new()
        }
    }

    pub fn get_arma(&self) -> &Arma {
        &self.arma
    }

    pub fn get_armadura(&self) -> &Armadura {
        &self.armadura
    }
}