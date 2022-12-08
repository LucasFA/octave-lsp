use logos::Logos;

/// The kind of a token produced by the lexer.
/// It is called this for consistency with Rowan, the parsing library.
#[derive(Logos, Debug, Copy, Clone, PartialEq)]
#[repr(u16)]
// #[logos(subpattern close_block_comment = r##"(#|%)\}"##)]
// #[logos(subpattern open_block_comment = r##"(#|%)\{"##)]
pub enum TokenKind {
    // This would be ideal for block comments, but logos doesn't support non-greedy regexes
    // #[regex(r##"(#|%)\{\s*\n(.|\n)*?(#|%)\}"##)]
    // Maybe we can use callbacks?
    // The current solution matches a bit more than it should, but it actually allows us to parse better
    // It also matches block comments that end in "#} foobar" (which is a line which should not end it, technically, but is most likely user error)
    // #[regex(r##"(.*[#%]\{.*\n)((?:[^#%]|[#%][^}])*)([#%]\})"##)] well this breaks everything
    #[regex("[#%].*")]
    Comment,

    #[regex("[ \n\r\t]+")]
    Whitespace,

    #[token("function")]
    FnKw,
    #[token("endfunction")]
    EndFnKw,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("=")]
    Equals,

    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    // The name of a variable must be a sequence of letters, digits and underscores, but it may not begin with a digit.
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    // A number is a sequence of digits, possibly containing a decimal point.
    #[regex(r"\d+")] // Integers
    #[regex(r"\d*\.\d+")] // Floats
    Number,

    #[error]
    Error,

    #[doc(hidden)]
    __LAST,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Lexer, Token};

    // snip
}
