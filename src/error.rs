use std::error::Error;

#[derive(Debug)]
pub(crate) enum ValidationError {
    TooManyElements,
    MultipleMemories,
}
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl Error for ValidationError {}
