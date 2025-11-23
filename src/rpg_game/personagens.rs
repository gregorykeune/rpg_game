use std::collections::HashMap;

use uuid::Uuid;
use crate::rpg_game::itens::{Arma, Armadura, ItemTipo};

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

#[derive(Clone)]

pub enum Classe {
    Guerreiro,
    Mago,
    Tanque,
}

impl Classe {
    pub fn as_str(&self) -> &str {
        match self {
            Classe::Guerreiro => "Guerreiro",
            Classe::Mago => "Mago",
            Classe::Tanque => "Tanque",
        }
    }
}

impl Personagem {
    pub fn get_arma(&self) -> &Arma {
        &self.arma
    }

    pub fn get_armadura(&self) -> &Armadura {
        &self.armadura
    }
}