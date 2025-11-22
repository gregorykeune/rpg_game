use std::collections::HashMap;
use uuid::Uuid;

use crate::rpg_game::{itens::ItemTipo, personagens::Personagem};

pub mod personagens;
pub mod itens;


struct Game {
    personagens: HashMap<Uuid, Personagem>,
    inventario: HashMap<Uuid, ItemTipo>,
    
}