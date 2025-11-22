use std::collections::HashMap;

use uuid::Uuid;
use crate::rpg_game::itens::{Arma, Armadura, ItemTipo};

pub struct Personagem {
    id: Uuid,
    nome: String,
    vida: u32,
    forca:u32,
    // decidi implementar o invetario no struct Game, usando 1 inventario universal para todos os personagens
    armadura: Armadura,
    arma: Arma,
    classe: Classe,
}

pub enum Classe {
    Guerreiro,
    Mago,
    Tanque,
    Arqueiro,
    Suporte,
}

impl Classe {
    pub fn as_str(&self) -> &str {
        match self {
            Classe::Guerreiro => "Guerreiro",
            Classe::Mago => "Mago",
            Classe::Tanque => "Tanque",
            Classe::Arqueiro => "Arqueiro",
            Classe::Suporte => "Suporte",
        }
    }
}