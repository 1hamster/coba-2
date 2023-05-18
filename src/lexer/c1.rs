use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {

    #[regex("//.*", logos::skip)]
    #[regex("/\\*[^(*/)]*\\*/", logos::skip)]
    Comment,

    #[token("bool")]
    KwBoolean,

    #[token("do")]
    KwDo,

    #[token("if")]
    KwIf,

    #[token("return")]
    KwReturn,

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

    #[token(r"*")]
    Asterisk,

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
<<<<<<< HEAD

=======
>>>>>>> main
    #[regex("[0-9]+")]
    ConstInt,

    #[regex(r"[0-9]*\.[0-9]+([eE]([+-])?([0-9])+)?|([0-9]+[eE]([+-])?([0-9])+)")]
    ConstFloat,

    #[regex("true|false")]
    ConstBoolean,

    #[regex("\"[^\n\"]*\"")]
    ConstString,
<<<<<<< HEAD

    #[regex("[a-zA-Z]+([0-9]|[a-zA-Z])*")]
    Id,

    //Pseudotoken
=======
>>>>>>> main

    #[regex("[a-zA-Z]+([a-zA-Z]|[0-9])*")]
    Id,

    #[regex(r"/\*([^*]|(\*+[^*/]))*\*+/")]
    Comment,
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
<<<<<<< HEAD
    #[regex(r"[ \r\t\n\f]+", logos::skip)]
=======
    #[regex(r"[ \t\n\f]+", logos::skip)]
>>>>>>> main
    Error,
}