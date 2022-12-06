use logos::Logos;


// fn parInteger(lex: &mut Lexer<Token>) -> Option<u64> {
//     let slice = lex.slice();
//     let n = slice.parse().ok()?;
//     Some(n)
// }

// TODO: handle single quotes for trasposed matrices
// TODO: handle comments
// TODO: handle scientific notation

#[derive(Logos, Debug, PartialEq)]
enum MyLexer {
    #[regex(r"\d+\.\d+")]
    Float,

    #[regex(r"\d+")]
    Integer,

    #[regex(r"'[^']+'")]
    #[regex(r#""[^"]+""#)]
    String,

    #[token("=")]
    Equal,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    ForwardSlash,

    #[token("^")]
    Caret,

    #[token(";")]
    Semicolon,

    #[regex(r"[a-zA-Z_]\w*")]
    Ident,

    #[regex(r"[ \t\n\r]+")]
    Whitespace,

    #[error]
    Error,
}

fn main() {
    let octave_code = r#"x = 3.14;
    y = 'hello world';
    t = "Hello world";
    z = x + y;
    result = z / 2.0;
    result = result ^ 2;"#;

    let mut lexer = MyLexer::lexer(octave_code);

    loop {
        let token = lexer.next();
        match token {
            Some(MyLexer::Float) => println!("number: {}", lexer.slice()),
            Some(MyLexer::Integer) => println!("Integer: {}", lexer.slice()),
            Some(MyLexer::String) => println!("string: {}", lexer.slice()),
            Some(MyLexer::Equal) => println!("equal"),
            Some(MyLexer::Plus) => println!("plus"),
            Some(MyLexer::Minus) => println!("minus"),
            Some(MyLexer::Asterisk) => println!("asterisk"),
            Some(MyLexer::ForwardSlash) => println!("forward slash"),
            Some(MyLexer::Caret) => println!("caret"),
            Some(MyLexer::Ident) => println!("ident: {}", lexer.slice()),
            Some(MyLexer::Whitespace) => (),
            Some(MyLexer::Semicolon) => println!("Semicolon"),
            Some(MyLexer::Error) => println!("error: {}", lexer.slice()),
            None => break,
        }
    }
}
