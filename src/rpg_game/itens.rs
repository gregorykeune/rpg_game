use uuid::Uuid;

use crate::{rpg_game::personagens::{self, Classe}, traits::ItemComportamento, errors::ErroRPG};

pub enum ItemTipo {
    Arma(Arma),
    Armadura(Armadura),
    Consumivel(Consumivel),
}

impl ItemComportamento for ItemTipo {
    fn get_nome(&self) -> String {
        match self {
            ItemTipo::Arma(arma) => arma.nome.clone(),
            ItemTipo::Armadura(armadura) => armadura.nome.clone(),
            ItemTipo::Consumivel(consumivel) => consumivel.nome.clone(),
        }
    }

    fn get_id(&self) -> Uuid {
        match self {
            ItemTipo::Arma(arma) => arma.id.clone(),
            ItemTipo::Armadura(armadura) => armadura.id.clone(),
            ItemTipo::Consumivel(consumivel) => consumivel.id.clone(),
        }
    }

    fn exibir_descricao(&self) -> String {
        match self {
            ItemTipo::Arma(arma) => {
                format!("Nome: {} \nDano: {} \nClasse: {} \nEfeito: {} \nRaridade: {} \nDurabilidade: {}/{}", arma.nome, arma.dano, arma.classe.as_str(), arma.efeito.as_string(), arma.raridade, arma.durabilidade_atual, arma.durabilidade_total) 
            },
            ItemTipo::Armadura(armadura) => format!("Nome: {} \nDefesa: {} \nRaridade: {} \nDurabilidade: {}/{}", armadura.nome, armadura.defesa, armadura.raridade, armadura.durabilidade_atual, armadura.durabilidade_total),
            ItemTipo::Consumivel(consumivel) => format!("Nome: {} \nEfeito: {} \nDescricao: {}", consumivel.nome, consumivel.efeito.as_string(), consumivel.descricao),
        }
    }

    fn get_tipo(&self) -> &str {
        match self {
            ItemTipo::Arma(_) => "Arma",
            ItemTipo::Armadura(_) => "Armadura",
            ItemTipo::Consumivel(_) => "Consumivel",
        }
    }

    fn usar(&self, personagem: &mut personagens::Personagem) -> Result<bool, ErroRPG> {
        return Err(ErroRPG::ItemNaoEncontrado(()));
    }
}


pub struct Arma {
    id: Uuid,
    nome: String,
    dano: u32,
    classe: Classe,
    efeito: Efeito,
    raridade: String,
    durabilidade_total: u32,
    durabilidade_atual: u32,
}

pub struct Armadura {
    id: Uuid,
    nome: String,
    defesa: u32,
    raridade: String,
    durabilidade_total: u32,
    durabilidade_atual: u32,
}

pub struct Consumivel {
    id: Uuid,
    nome: String,
    efeito: Efeito,
    descricao: String,
}

pub enum Efeito {
    Fisico, //dano normal
    Congelamento, //ataque de congelamento adiciona 1 ponto de congelamento, com 3 pontos o oponente fica congelado e perde a vez
    Queimadura(u32, u32), // recebe um dano definido por um numero definido de rodadas 
    Veneno(u32), // recebe um dano percentual até o final da batalha
    Eletricidade(u32, u32), // o personagem com esse efeito, ao tentar atacar tem chance de tomar choque e errar o ataque
    Sangramento(u32, u32), // o golpe tem chance de dar dano de sangramento (dano critico)
    Cura(u32), // recebe vida
    Enfraquecimento(u32), // o dano causado pelo personagem com esse efeito é reduzido
}

impl Efeito {
    pub fn as_string(&self) -> String {
        match self {
            Efeito::Fisico => "Fisico".to_string(),
            Efeito::Congelamento => "Congelamento".to_string(),
            Efeito::Queimadura(dano, rodadas) => format!("Queimadura; Dano: {dano}; Rodadas: {rodadas}", ),
            Efeito::Veneno(dano_percentual) => format!("Veneno; Dano por rodada: {}%", dano_percentual),
            Efeito::Eletricidade(dano_choque, probabilidade) => format!("Eletricidade; Dano do choque: {}; Probabilidade: {}%", dano_choque, probabilidade),
            Efeito::Sangramento(dano, probabilidade) => format!("Sangramento; Dano: {}%; Probabilidade: {}%", dano, probabilidade),
            Efeito::Cura(cura) => format!("Cura; Pontos de vida: {}", cura),
            Efeito::Enfraquecimento(reducao_dano) => format!("Enfraquecimento; Dano reduzido: {}%", reducao_dano),
        }
    }
}
