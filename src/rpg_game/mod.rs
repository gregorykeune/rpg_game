use std::{collections::HashMap, fs::File, io::{self, BufReader, BufWriter, Write}, path::Path};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::{errors::ErroRPG, rpg_game::{itens::{Armadura, ItemTipo}, personagens::{Classe, Personagem}}, traits::ItemComportamento};

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

    fn criar_personagem(&self, simulacao: bool) {
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
            }
        
        let armadura = 

            
        }
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

    fn procurar_item_nome(&self, nome: String) -> Result<&ItemTipo, ErroRPG> {
        self.itens
            .iter()
            .find(|(_, item)| item.get_nome() == nome)
            .map(|(_, item)| item)
            .ok_or_else(|| ErroRPG::ItemNaoEncontrado(nome))
    }
        
}


fn ler_classe() -> Result<Classe, ErroRPG> {
    let opcao = 3;
    while opcao > 2 {
        let opcao = ler_u32_loop(
            "Qual sera a classe do seu personagem?
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

