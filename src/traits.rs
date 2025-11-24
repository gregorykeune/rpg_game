use std::collections::HashMap;

use uuid::Uuid;
use crate::rpg_game::personagens::Personagem;
use crate::errors::ErroRPG;

pub trait ItemComportamento {
    fn get_nome(&self) -> String;

    fn get_id(&self) -> Uuid;

    fn get_tipo(&self) -> &str;

    fn exibir_descricao(&self) -> String;

    fn usar(&self, personagem: &mut Personagem) -> Result<bool, ErroRPG>;
}

pub trait Identificavel {
    fn id(&self) -> Uuid;

    fn buscar_item_por_id<'a, T>(colecao: &'a HashMap<Uuid, T>, id: &Uuid) -> Option<&'a T> {
        colecao.get(id)
    }

    fn buscar_em<'a, T>(colecao: &'a HashMap<Uuid, T>, id: &Uuid) -> Option<&'a T>;
}