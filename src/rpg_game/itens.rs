use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::{errors::ErroRPG, rpg_game::personagens::{Classe, Personagem}, traits::ItemComportamento};

#[derive(Clone, Serialize, Deserialize)]
pub enum ItemTipo {
    Arma(Arma),
    Armadura(Armadura),
    Consumivel(Consumivel),
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Arma {
    id: Uuid,
    nome: String,
    dano: u32,
    classe: Classe,
    efeito: Efeito,
    raridade: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Armadura {
    id: Uuid,
    nome: String,
    defesa: u32,
    raridade: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Consumivel {
    id: Uuid,
    nome: String,
    efeito_vida: i32,
    descricao: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Efeito {
    Fisico, //dano normal
    Congelamento, //ataque de congelamento adiciona 1 ponto de congelamento, com 3 pontos o oponente fica congelado e perde a vez
    Queimadura(u32, u32), // recebe um dano definido por um numero definido de rodadas 
    Veneno(u32), // recebe um dano percentual até o final da batalha
    Eletricidade(u32, u32), // o personagem com esse efeito, ao tentar atacar tem chance de tomar choque e errar o ataque
    Sangramento(u32, u32), // o golpe tem chance de dar dano de sangramento (dano critico)
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
            
            Efeito::Enfraquecimento(reducao_dano) => format!("Enfraquecimento; Dano reduzido: {}%", reducao_dano),
        }
    }
}

impl Armadura {
    pub fn new(nome: String, defesa: u32, raridade: String) -> Self {
        Armadura {
            id: Uuid::new_v4(),
            nome,
            defesa,
            raridade,
        }
    }

    pub fn get_defesa(&self) -> u32 {
        self.defesa
    }
}

impl Arma {
    pub fn new(nome: String, dano: u32, classe: Classe, raridade: String, efeito: Efeito) -> Self {
        Arma {
            id: Uuid::new_v4(),
            nome,
            dano,
            classe,
            efeito,
            raridade,
        }
    }
}


impl Consumivel {
    pub fn new(nome: String, efeito_vida: i32, descricao: String) -> Self {
        Consumivel {
            id: Uuid::new_v4(),
            nome,
            efeito_vida,
            descricao,
        }
    }
}
//implementacao de ItemComportamento

impl ItemComportamento for Armadura {
    fn get_nome(&self) -> String {
        self.nome.clone()
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_tipo(&self) -> &str {
        "Armadura"
    }

    fn exibir_descricao(&self) -> String {
        format!(
            "Nome: {} \nDefesa: {} \nRaridade: {}", self.nome, self.defesa, self.raridade,
        )
    }
    

    fn usar(&self, personagem: &mut Personagem) -> Result<bool, ErroRPG> {
        if personagem.inventario.contains_key(&self.id) {
            return Err(ErroRPG::ItemNaoEncontrado(self.id));
        }

        if personagem.armadura.get_nome() == self.get_nome() {
            return Err(ErroRPG::ItemNaoUsavel("Esse item ja esta sendo usado".to_string()));
        }

        if personagem.armadura.get_nome() != "Nenhuma" {
            personagem.inventario.insert(personagem.armadura.get_id(), ItemTipo::Armadura(personagem.armadura.clone()));
        }

        personagem.armadura = self.clone();
        personagem.defesa = self.defesa;

        Ok(true)
    }
}

impl ItemComportamento for Arma {
    fn get_nome(&self) -> String {
        self.nome.clone()
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_tipo(&self) -> &str {
        "Arma"
    }

    fn exibir_descricao(&self) -> String {
        format!(
            "Nome: {} \nDano: {} \nClasse: {} \nEfeito: {} \nRaridade: {}", self.nome, self.dano, self.classe.as_str(), self.efeito.as_string(), self.raridade
        )
    }
    

    fn usar(&self, personagem: &mut Personagem) -> Result<bool, ErroRPG> {
        if personagem.inventario.contains_key(&self.id) {
            return Err(ErroRPG::ItemNaoEncontrado(self.id));
        }

        if personagem.arma.get_nome() == self.get_nome() {
            return Err(ErroRPG::ItemNaoUsavel("Esse item ja esta sendo usado".to_string()));
        }

        if personagem.arma.classe.as_str() != self.classe.as_str() {
            return Err(ErroRPG::ArmaIncompativel("A classe da arma é diferente da classe do personagem!".to_string()));
        }

        if personagem.arma.get_nome() != "Nenhuma" {
            personagem.inventario.insert(personagem.arma.get_id(), ItemTipo::Arma(personagem.arma.clone()));
        }

        personagem.arma = self.clone();

        Ok(true)
    }
}

impl ItemComportamento for Consumivel {
    fn get_nome(&self) -> String {
        self.nome.clone()
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_tipo(&self) -> &str {
        "Consumivel"
    }

    fn exibir_descricao(&self) -> String {
        format!(
            "Nome: {} \nEfeito: {} \nDescricao: {}", self.nome, self.efeito_vida, self.descricao
        )
    }
    
    fn usar(&self, personagem: &mut Personagem) -> Result<bool, ErroRPG> {
        if personagem.inventario.contains_key(&self.id) {
            return Err(ErroRPG::ItemNaoEncontrado(self.id));
        }

        if personagem.vida <= 0 {
            if self.nome != "Revive" {
                return Err(ErroRPG::ItemNaoUsavel("O personagem ja esta morto".to_string()));
            }
            personagem.vida = self.efeito_vida as u32;
        } else {
            personagem.vida += self.efeito_vida as u32;
        }

        Ok(true)
    }
}

impl ItemComportamento for ItemTipo {
    fn get_nome(&self) -> String {
        match self {
            ItemTipo::Arma(arma) => arma.get_nome(),
            ItemTipo::Armadura(armadura) => armadura.get_nome(),
            ItemTipo::Consumivel(consumivel) => consumivel.get_nome(),
        }
    }

    fn get_id(&self) -> Uuid {
        match self {
            ItemTipo::Arma(arma) => arma.get_id(),
            ItemTipo::Armadura(armadura) => armadura.get_id(),
            ItemTipo::Consumivel(consumivel) => consumivel.get_id(),
        }
    }

    fn exibir_descricao(&self) -> String {
        match self {
            ItemTipo::Arma(arma) => arma.exibir_descricao(),
            ItemTipo::Armadura(armadura) => armadura.exibir_descricao(),
            ItemTipo::Consumivel(consumivel) => consumivel.exibir_descricao(),
        }
    }

    fn get_tipo(&self) -> &str {
        match self {
            ItemTipo::Arma(a) => a.get_tipo(),
            ItemTipo::Armadura(a) => a.get_tipo(),
            ItemTipo::Consumivel(c) => c.get_tipo(),
        }
    }

    fn usar(&self, personagem: &mut Personagem) -> Result<bool, ErroRPG> {
        match self {
            ItemTipo::Arma(a) => a.usar(personagem),
            ItemTipo::Armadura(a) => a.usar(personagem),
            ItemTipo::Consumivel(c) => c.usar(personagem),
        }
    }
}