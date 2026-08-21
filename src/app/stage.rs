#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Welcome,
    Playing,
    #[allow(dead_code)] // alcanzable desde la etapa 3 al llegar a la meta
    Success,
}
