//! Example on how to use the [`Layouter::embed_with_visualize_with_source`] to embed a tree and
//! visualize it with the source code.
//! We use the data from the syntree crate's example `calculator` -- "256 / 2 + 64 * 2"
//! The tree has the following structure:
//! ```text
//! # Tree:
//! Operation@0..16
//!   Number@0..3
//!     Number@0..3 "256"
//!   Whitespace@3..4 " "
//!   Operator@4..5
//!     Div@4..5 "/"
//!   Whitespace@5..6 " "
//!   Number@6..7
//!     Number@6..7 "2"
//!   Whitespace@7..8 " "
//!   Operator@8..9
//!     Plus@8..9 "+"
//!   Whitespace@9..10 " "
//!   Operation@10..16
//!     Number@10..12
//!       Number@10..12 "64"
//!     Whitespace@12..13 " "
//!     Operator@13..14
//!       Mul@13..14 "*"
//!     Whitespace@14..15 " "
//!     Number@15..16
//!       Number@15..16 "2"
//!```
use std::fmt::Display;

use anyhow::Result;
use syntree_layout::Layouter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
enum Syntax {
    Number,
    Plus,
    Div,
    Mul,
    Whitespace,
    Operator,
    Operation,
}

impl Display for Syntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Syntax::Number => "Number",
            Syntax::Plus => "Plus",
            Syntax::Div => "Div",
            Syntax::Mul => "Mul",
            Syntax::Whitespace => "Whitespace",
            Syntax::Operator => "Operator",
            Syntax::Operation => "Operation",
        };
        write!(f, "{}", s)
    }
}

fn main() -> Result<()> {
    let source = "256 / 2 + 64 * 2";
    let tree = syntree::tree! {
        Syntax::Operation => {
            Syntax::Number => {
                (Syntax::Number, 3),
            },
            (Syntax::Whitespace, 1),
            Syntax::Operator => {
                (Syntax::Div, 1),
            },
           (Syntax::Whitespace, 1),
            Syntax::Number => {
                (Syntax::Number, 1),
            },
            (Syntax::Whitespace, 1),
            Syntax::Operator => {
                (Syntax::Plus, 1),
            },
            (Syntax::Whitespace, 1),
            Syntax::Operation => {
                Syntax::Number => {
                    (Syntax::Number, 2),
                },
                (Syntax::Whitespace, 1),
                Syntax::Operator => {
                    (Syntax::Mul, 1),
                },
                (Syntax::Whitespace, 1),
                Syntax::Number => {
                    (Syntax::Number, 1),
                },
            },

        }
    };

    // Embed the tree with the source code.
    Layouter::new(&tree)
        .with_file_path("examples/example3_1.svg")
        .embed_with_source(source)
        .map_err(|e| anyhow::anyhow!(e))?
        .write()
        .map_err(|e| anyhow::anyhow!(e))?;
    // Embed the tree with the source code and `Display` trait.
    Layouter::new(&tree)
        .with_file_path("examples/example3_2.svg")
        .embed_with_source_and_display(source)
        .map_err(|e| anyhow::anyhow!(e))?
        .write()
        .map_err(|e| anyhow::anyhow!(e))
}
