use std::{collections::HashMap, path::PathBuf};
use uuid::Uuid;

use crate::rpg_game::{itens::ItemTipo, personagens::Personagem};

pub mod personagens;
pub mod itens;


struct Game {
    personagens: HashMap<Uuid, Personagem>,
    persistencia_path: PathBuf,    
}


