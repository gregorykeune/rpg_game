use uuid::Uuid;

#[derive(Debug)]
pub enum ErroRPG {
    PersonagemNaoEncontrado(Uuid),
    ItemNaoEncontrado(String), 
    ErroPersistencia(String), 
    EntradaInvalida(String), 
    ItemNaoUsavel(String),
    ArmaIncompativel(String),
    ClasseInvalida,
    EfeitoInvalido,
}