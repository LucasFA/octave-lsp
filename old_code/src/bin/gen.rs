extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate ron;
extern crate file;

use std::path::PathBuf;
use std::fmt::Write;

fn main() {
    let grammar = Grammar::read();
    let text = grammar.to_tokens();
    file::put_text(&generated_file(), &text).unwrap();
}

#[derive(Deserialize)]
struct Grammar {
    syntax_kinds: Vec<String>,
    operators: Vec<String>,
    keywords: Vec<String>,
}

impl Grammar {
    fn read() -> Grammar {
        let text = file::get_text(&grammar_file()).unwrap();
        ron::de::from_str(&text).unwrap()
    }

    fn to_tokens(&self) -> String {
        let mut acc = String::new();
        acc.push_str("// File generated from grammar.ron by executing gen.rs\n");
        acc.push_str("use crate::tree::{SyntaxKind, SyntaxInfo};\n");
        acc.push_str("use logos::Logos;\n");
        acc.push_str("\n");

        acc.push_str("#[derive(Logos, Debug, PartialEq)]\n");
        acc.push_str("enum MyLexer {\n");

        acc.push_str("    // Keywords:\n");
        for keyword in &self.keywords {
            acc.push_str("    #[token({keyword})]");
            acc.push_str(keyword);
            acc.push_str("\n\n");
        }

        // for (idx, kind) in self.syntax_kinds.iter().enumerate() {
        //     let sname = all_uppercase(kind);
        //     write!(
        //         acc,
        //         "pub const {}: SyntaxKind = SyntaxKind({});\n",
        //         sname, idx
        //     ).unwrap();
        // }
        acc.push_str("\n");
        for kind in self.syntax_kinds.iter() {
            let sname = all_uppercase(kind);
            write!(
                acc,
                "static {sname}_INFO: SyntaxInfo = SyntaxInfo {{\n   name: \"{sname}\",\n}};\n",
                sname = sname
            ).unwrap();
        }
        acc.push_str("\n");

        acc.push_str("pub(crate) fn syntax_info(kind: SyntaxKind) -> &'static SyntaxInfo {\n");
        acc.push_str("    match kind {\n");
        for kind in self.syntax_kinds.iter() {
            let sname = all_uppercase(kind);
            write!(
                acc,
                "        {sname} => &{sname}_INFO,\n",
                sname = sname
            ).unwrap();
        }
        acc.push_str("        _ => unreachable!()\n");
        acc.push_str("    }\n");
        acc.push_str("}\n");
        acc
    }
}

fn grammar_file() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(dir).join("grammar.ron")
}

fn generated_file() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(dir).join("src/syntax_kinds.rs")
}

fn all_uppercase(word: &str) -> String {
    word.chars().map(|c| c.to_ascii_uppercase()).collect()
}