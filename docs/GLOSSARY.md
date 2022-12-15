# Overview

The client launches the server, which communicate through the LSP. The server then starts analyzing open files.

To do so, it first uses the grammar of the language to identify the meaning of each token in the language. See the 'lexer.rs' file.

Then, identifies the meaning of each expression in the language. See the 'parser.rs' file.


## Glossary

- **AST**: Abstract Syntax Tree
- **Parser**: The parser is the part of the compiler that takes the tokens and turns them into an AST.
- **Lexer**: The lexer is the part of the compiler that takes the source code and turns it into tokens.
- **Token**: A token is a single unit of syntax. For example, in the expression `1 + 2`, the tokens are `1`, `+`, and `2`, and the token type is `Number`, `Plus`, and `Number`. Note the token type is part of the token.
- **Lexeme**: The lexeme is the actual text of the token. For example, in the expression `1 + 2`, the lexemes are `1`, `+`, and `2`.
- **Span**: The span is the location of the token in the source code. For example, in the expression `1 + 2`, the spans are `0..1`, `2..3`, and `4..5`, forgoing the spaces. Note that we do take into account whitespace.