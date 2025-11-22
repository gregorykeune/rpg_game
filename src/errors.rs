use uuid::Uuid;

pub enum ErroRPG {
    PersonagemNaoEncontrado(Uuid),
    ItemNaoEncontrado(Uuid), 
    ErroPersistencia(String), 
    EntradaInvalida(String), 
    ItemNaoUsavel(String),
}