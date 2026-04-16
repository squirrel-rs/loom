/// Imports
use crate::error::RuntimeError;
use loom_common::bail;
use loom_lex::token::Span;

/// Bails runtime error with provided span and text
pub fn error(span: &Span, text: &str) -> ! {
    bail!(RuntimeError::Bail {
        text: text.to_string(),
        src: span.0.clone(),
        span: span.1.clone().into()
    })
}
