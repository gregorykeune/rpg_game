use uuid::Uuid;

pub enum ErroRPG {
    PersonagemNaoEncontrado(Uuid),
    ItemNaoEncontrado(String), 
    ErroPersistencia(String), 
    EntradaInvalida(String), 
    ItemNaoUsavel(String),
    ArmaIncompativel(String),
    ClasseInvalida,
}