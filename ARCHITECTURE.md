# Overview

The client launches the server, which communicate through the LSP. The server then starts analyzing open files.

To do so, it first uses the grammar of the language to identify the meaning of each token in the language. See the 'lexer.rs' file.

Then, identifies the meaning of each expression in the language. See the 'parser.rs' file.