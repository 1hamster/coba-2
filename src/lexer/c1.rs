use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // TODO: Define variants and their token/regex
    // Keywords:
    #[token("bool")]
    KwBoolean,

    #[token("do")]
    KwDo,
    
    #[token("else")]
    KwElse,

    #[token("float")]
    KwFloat,

    #[token("for")]
    KwFor,

    #[token("int")]
    KwInt,

    #[token("printf")]
    KwPrintf,

    #[token("void")]
    KwVoid,

    #[token("while")]
    KwWhile,


    // Operators
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,
    
    #[token("*")]
    Asteriks,

    #[token("/")]
    Slash,

    #[token("=")]
    Assign,

    #[token("==")]
    Eq,

    #[token("!=")]
    Neq,

    #[token("<")]
    Lss,

    #[token(">")]
    Grt,
    
    #[token("<=")]
    Leq,
    
    #[token(">=")]
    Geq,

    #[token("&&")]
    And,

    #[token("||")]
    Or,


    // Misc
    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,
    
    #[token("(")]
    LParen,
    
    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,


    // Termvariablen
    #[regex("", Integer)]
    ConstInt,



    // Pseudotoken
    // #[regex("[0-9]")]
    // Digit,

    // #[regex("D+")]
    // Integer,

    // #[regex("{Integer} "." {Integer} | "." {Integer}")]
    // Float,


    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}
