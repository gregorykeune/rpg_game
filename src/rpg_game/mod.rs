use std::{collections::HashMap, fs::File, io::{self, BufReader, BufWriter, Write}, path::Path};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::{errors::ErroRPG, rpg_game::{itens::{Arma, Armadura, Consumivel, Efeito, ItemTipo}, personagens::{Classe, Personagem}}, traits::{Identificavel, ItemComportamento}};

pub mod personagens;
pub mod itens;


#[derive(Serialize, Deserialize)]
pub struct Game {
    personagens: HashMap<Uuid, Personagem>,
    #[serde(skip)]
    itens: HashMap<Uuid, ItemTipo>,
    persistencia_path: std::path::PathBuf,    
}

impl Game {
    /// Carrega os dados do jogo de um arquivo JSON.
    /// Se o arquivo não existir, retorna um novo Game vazio.
    pub fn carregar(path: impl AsRef<Path>) -> Result<Self, ErroRPG> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Ok(Self {
                personagens: HashMap::new(),
                persistencia_path: path.to_path_buf(),
                itens: HashMap::new()
            });
        }

        let file = File::open(path)
            .map_err(|e| ErroRPG::ErroPersistencia(format!("Erro ao abrir arquivo: {}", e)))?;
        
        let reader = BufReader::new(file);
        
        let mut game: Game = serde_json::from_reader(reader)
            .map_err(|e| ErroRPG::ErroPersistencia(format!("Erro ao deserializar JSON: {}", e)))?;
        
        game.persistencia_path = path.to_path_buf();
        
        Ok(game)
    }

    /// Salva o estado atual do jogo em um arquivo JSON.
    /// Cria o arquivo se não existir ou sobrescreve se já existir.
    pub fn salvar(&self) -> Result<(), ErroRPG> {
        let file = File::create(&self.persistencia_path)
            .map_err(|e| ErroRPG::ErroPersistencia(format!("Erro ao criar arquivo: {}", e)))?;
        
        let writer = BufWriter::new(file);
        
        serde_json::to_writer_pretty(writer, self)
            .map_err(|e| ErroRPG::ErroPersistencia(format!("Erro ao serializar JSON: {}", e)))?;
        
        Ok(())
    }


    //=============================================================================================================================
    //================================================= FUNÇÕES ===================================================================
    //=============================================================================================================================


    //DEVO CHAMAR ESSA FUNÇÃO NA MAIN

    // pub fn iniciar_menu(&self) {
    //     println!("================= MENU PRINCIPAL ================= \n[1] Teste/Simulcao \n[2] Novo Jogo \n[3] Continuar Jogo (criara um novo caso não tenha um save)");

    //     let entrada = ler_string();

    //     match entrada {
    //         "1" => {
                
    //         },
    //         "2" => {

    //         }
    //         "3" => {

    //         }
    //     } 
    // }

    pub fn listar_inventario(&self, personagem: Personagem) {
        println!("=========== ARMADURAS ===========");

        let mut encontrou = false;

        for item in personagem.inventario.values() {
            if let ItemTipo::Armadura(armadura) = item {
                encontrou = true;
                println!(
                    "ID: {} \nNome: {}, Defesa: {}, Raridade: {}",
                    armadura.id(),
                    armadura.get_nome(),
                    armadura.get_defesa(),
                    armadura.get_raridade()
                );
                println!("---------------------------------------------------");
            }
        }

        if !encontrou {
            println!("Nenhuma armadura encontrada no inventario.");
        }

        println!("============= ARMAS =============");

        let mut encontrou = false;

        for item in personagem.inventario.values() {
            if let ItemTipo::Arma(arma) = item {
                encontrou = true;
                println!(
                    "ID: {} \nNome: {}, Dano: {}, Raridade: {}, Efeito: {}",
                    arma.id(),
                    arma.get_nome(),
                    arma.get_dano(),
                    arma.get_raridade(),
                    arma.get_efeito().as_string(),
                );
                println!("---------------------------------------------------");
            }
        }

        if !encontrou {
            println!("Nenhuma arma encontrada no inventario.");
        }

        println!("=========== CONSUMIVEIS ===========");

        let mut encontrou = false;

        for item in personagem.inventario.values() {
            if let ItemTipo::Consumivel(consumivel) = item {
                encontrou = true;
                println!(
                    "ID: {} \nNome: {}, Cura: {}, Descricao: {}",
                    consumivel.id(),
                    consumivel.get_nome(),
                    consumivel.get_efeito_vida(),
                    consumivel.get_descricao(),
                );
                println!("---------------------------------------------------");
            }
        }

        if !encontrou {
            println!("Nenhuma consumivel encontrado no inventario.");
        }
        
    }

    pub fn listar_todas_armaduras(&self) {
        println!("=========== ARMADURAS ===========");

        let mut encontrou = false;

        for item in self.itens.values() {
            if let ItemTipo::Armadura(armadura) = item {
                encontrou = true;
                println!(
                    "ID: {} \nNome: {}, Defesa: {}, Raridade: {}",
                    armadura.id(),
                    armadura.get_nome(),
                    armadura.get_defesa(),
                    armadura.get_raridade()
                );
                println!("---------------------------------------------------");
            }
        }

        if !encontrou {
            println!("Nenhuma armadura encontrada.");
        }
    }

    pub fn listar_todas_armas(&self) {
        println!("============= ARMAS =============");

        let mut encontrou = false;

        for item in self.itens.values() {
            if let ItemTipo::Arma(arma) = item {
                encontrou = true;
                println!(
                    "ID: {} \nNome: {}, Dano: {}, Raridade: {}, Efeito: {}",
                    arma.id(),
                    arma.get_nome(),
                    arma.get_dano(),
                    arma.get_raridade(),
                    arma.get_efeito().as_string(),
                );
                println!("---------------------------------------------------");
            }
        }

        if !encontrou {
            println!("Nenhuma arma encontrada.");
        }
    }

    pub fn listar_todos_consumiveis(&self) {
        println!("=========== CONSUMIVEIS ===========");

        let mut encontrou = false;

        for item in self.itens.values() {
            if let ItemTipo::Consumivel(consumivel) = item {
                encontrou = true;
                println!(
                    "ID: {} \nNome: {}, Cura: {}, Descricao: {}",
                    consumivel.id(),
                    consumivel.get_nome(),
                    consumivel.get_efeito_vida(),
                    consumivel.get_descricao(),
                );
                println!("---------------------------------------------------");
            }
        }

        if !encontrou {
            println!("Nenhuma consumivel encontrado.");
        }
    }

    fn criar_personagem(&mut self, simulacao: bool) {
        println!("================= CRIAR PERSONAGEM =====================");
        println!("Nome: ");
        
        let nome = ler_string();

        let vida;
        let forca;
        if let Ok(classe) = ler_classe() {
            if simulacao {
                vida = ler_u32_loop("Pontos de vida: ");
                forca = ler_u32_loop("Pontos de forca: ");

            }
            else {
                (vida, forca) = self.identificar_vida_forca(classe);
                let armadura = self.armadura_padrao();
            }
        


        
            
        }
    }

    fn armadura_simulacao(&mut self) -> Armadura {
        let opcao = 3;
        let mut armadura;
        while opcao > 2 {   
            if opcao == 0 {
                let opcao = ler_u32_loop("[0] Criar uma armadura nova e equipar \n[1] Equipar uma armadura ja existente");
                armadura = self.criar_armadura();  
            }
            else if opcao == 1 {
                self.listar_todas_armaduras();
                let id = ler_uuid_loop("ID: ");
                let armadura: = Armadura::buscar_em(&self.itens, &id);
                    
            }
        }
    
        armadura
    }   

    fn criar_armadura(&mut self) -> Armadura {
        // nome defesa raridade
        println!("============= CRIAR ARMADURA =============");
        
        print!("Nome: ");
        let nome = ler_string();
        
        let defesa = ler_u32_loop("Defesa: ");
        
        print!("Raridade: ");
        let raridade = ler_string();

        let armadura = Armadura::new(nome, defesa, raridade);

        self.itens.insert(armadura.id(), ItemTipo::Armadura(armadura.clone()));

        armadura
    }

    fn criar_arma(&mut self) -> Arma {
        // nome defesa raridade
        println!("============= CRIAR ARMA =============");
        
        print!("Nome: ");
        let nome = ler_string();
        
        let dano = ler_u32_loop("Dano: ");
        
        let classe = ler_classe().unwrap();

        print!("Raridade: ");
        let raridade = ler_string();

        let efeito = ler_efeito().unwrap();


        let arma = Arma::new(nome, dano, classe, raridade, efeito);

        self.itens.insert(arma.id(), ItemTipo::Arma(arma.clone()));

        arma
    }

    fn criar_consumivel(&mut self) -> Consumivel {
        println!("=========== CRIAR CONSUMIVEL ===========");

        print!("Nome: ");
        let nome = ler_string();

        let efeito_vida = ler_u32_loop("Cura: ") as i32;
    
        print!("Descricao: ");
        let descricao = ler_string();

        let consumivel = Consumivel::new(nome, efeito_vida, descricao);
    
        self.itens.insert(consumivel.id(), ItemTipo::Consumivel(consumivel.clone()));

        consumivel
    }


    fn armadura_padrao(&mut self) -> Armadura {
        let armadura = match self.procurar_item_nome("Armadura de Couro".to_string()) {
            Ok(ItemTipo::Armadura(a)) => a, // já é um clone correto
            _ => {
                // criar nova e inserir no HashMap
                let nova = Armadura::new("Armadura de Couro".to_string(), 10, "Comum".to_string());

                self.itens.insert(nova.get_id(), ItemTipo::Armadura(nova.clone()));

                nova
            }
        };
        armadura
    }

    

    fn identificar_vida_forca(&self, classe: Classe) -> (u32, u32){
        let vida;
        let forca;
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

        return (vida, forca);
    }

    fn procurar_item_nome(&self, nome: String) -> Result<ItemTipo, ErroRPG> {
        self.itens
            .values()
            .find(|item| item.get_nome() == nome)
            .cloned() // ← devolve o ItemTipo inteiro
            .ok_or_else(|| ErroRPG::ItemNaoEncontrado(nome))
    }
        
}


fn ler_classe() -> Result<Classe, ErroRPG> {
    let opcao = 3;
    while opcao > 2 {
        let opcao = ler_u32_loop(
            "Classe:
            \n[0] Guerreiro \n[1] Mago \n[2] Assassino
            \nOpcao: "
        );
        if opcao > 2 {
            println!("O número inserido não se refere a nenhuma classe existente, tente novamente.");
        }
    }

    match opcao {
        0 => return Ok(Classe::Guerreiro), 
        1 => return Ok(Classe::Mago), 
        2 => return Ok(Classe::Assassino), 
        _ => return Err(ErroRPG::ClasseInvalida)
    }
}

fn ler_efeito() -> Result<Efeito, ErroRPG> {
    let opcao = 3;
    while opcao > 2 {
        let opcao = ler_u32_loop(
            "Efeito:
            \n[0] Fisico \n[1] Congelamento \n[2] Queimadura \n[3] Veneno \n[4] Eletricidade \n[5] Sangramento \n[6] Enfraquecimento
            \nOpcao: "
        );
        if opcao > 6 {
            println!("O número inserido não se refere a nenhum efeito existente, tente novamente.");
        }
    }

    match opcao {
        0 => return Ok(Efeito::Fisico), 
        1 => return Ok(Efeito::Congelamento), 
        2 => return {
            let dano = ler_u32_loop("Dano de queimadura: ");
            let num_rodadas = ler_u32_loop("Numero de rodadas do efeito: ");
            Ok(Efeito::Queimadura(dano, num_rodadas))
        }, 
        3 => {
            let dano = ler_f32_loop("Dano percentual por rodada: ");
            Ok(Efeito::Veneno(dano))
        },
        4 => {
            let dano = ler_u32_loop("Dano do choque: ");
            let probabilidade = ler_u32_loop("Percentual de chance de causar choque: ");
            Ok(Efeito::Eletricidade(dano, probabilidade))
        },
        5 => {
            let dano = ler_u32_loop("Dano do sangramento: ");
            let probabilidade = ler_u32_loop("Percentual de chance de causar choque: ");
            Ok(Efeito::Sangramento(dano, probabilidade))
        },
        6 => {
            let reducao_dano = ler_u32_loop("Percentual de reducao de dano: ")
            Ok(Efeito::Enfraquecimento(reducao_dano))
        }
        _ => return Err(ErroRPG::EfeitoInvalido)
    }
}

fn ler_string() -> String {
    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");

    entrada.trim().to_string()
}

pub fn ler_u32() -> Result<u32, ErroRPG> {
    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .map_err(|_| ErroRPG::EntradaInvalida("Erro ao ler entrada".to_string()))?;

    let valor = entrada
        .trim()
        .parse::<u32>()
        .map_err(|_| ErroRPG::EntradaInvalida("Entrada deve ser um número inteiro positivo".to_string()))?;

    Ok(valor)
}

pub fn ler_u32_loop(prompt: &str) -> u32 {
    loop {
        print!("{}", prompt);
        let _ = std::io::stdout().flush();

        match ler_u32() {
            Ok(n) => return n,
            Err(ErroRPG::EntradaInvalida(s)) => println!("{}", s),
            _ => println!("Erro desconhecido ao ler entrada"),
        }
    }
}

pub fn ler_f32() -> Result<f32, String> {
    let mut entrada = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).map_err(|_| "Erro ao ler entrada".to_string())?;

    entrada
        .trim()
        .replace(",", ".") // permite usar "1,5" ou "1.5"
        .parse::<f32>()
        .map_err(|_| "Valor inválido".to_string())
}

pub fn ler_f32_loop(msg: &str) -> f32 {
    loop {
        print!("{}", msg);
        std::io::stdout().flush().unwrap();

        match ler_f32() {
            Ok(num) => return num,
            Err(_) => println!("Valor inválido! Digite novamente."),
        }
    }
}

pub fn ler_uuid() -> Result<Uuid, ErroRPG> {
    let mut entrada = String::new();

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut entrada)
        .map_err(|_| ErroRPG::EntradaInvalida("Erro ao ler entrada".to_string()))?;

    let texto = entrada.trim();

    Uuid::parse_str(texto)
        .map_err(|_| ErroRPG::EntradaInvalida(format!("UUID inválido: {}", texto)))
}

pub fn ler_uuid_loop(msg: &str) -> Uuid {
    loop {
        print!("{}", msg);
        io::stdout().flush().unwrap();

        match ler_uuid() {
            Ok(uuid) => return uuid,
            Err(ErroRPG::EntradaInvalida(s)) => println!("{}", s),
            _ => println!("Erro desconhecido ao ler entrada"),
        }
    }
}
