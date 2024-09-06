use std::fmt;
use syntree_layout::{Layouter, Visualize};

enum Ast {
    Calc,
    CalcLst1,
    CalcLst1Itm1,
    Instruction,
    Assignment,
    AssignItem,
    Id,
    AssignOp,
    LocigalOr,
    LocigalAnd,
    BitwiseOr,
    BitwiseAnd,
    Equality,
    Relational,
    BitwiseShift,
    Sum,
    Mult,
    Power,
    Factor,
    Number,
    MultLst1,
    MultLst1Itm1,
    MultItem,
    MultOp,
    SumLst1,
    SumLst1Itm1,
    SumItem,
    AddOp,
    Plus,
    Tok(&'static str),
}

impl Visualize for Ast {
    fn visualize(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Ast::Calc => "calc",
            Ast::CalcLst1 => "calc_lst1",
            Ast::CalcLst1Itm1 => "calc_lst1_itm1",
            Ast::Instruction => "instruction",
            Ast::Assignment => "assignment",
            Ast::AssignItem => "assign_item",
            Ast::Id => "id",
            Ast::AssignOp => "assign_op",
            Ast::LocigalOr => "locigal_or",
            Ast::LocigalAnd => "locigal_and",
            Ast::BitwiseOr => "bitwise_or",
            Ast::BitwiseAnd => "bitwise_and",
            Ast::Equality => "equality",
            Ast::Relational => "relational",
            Ast::BitwiseShift => "bitwise_shift",
            Ast::Sum => "sum",
            Ast::Mult => "mult",
            Ast::Power => "power",
            Ast::Factor => "factor",
            Ast::Number => "number",
            Ast::MultLst1 => "mult_lst1",
            Ast::MultLst1Itm1 => "mult_lst1_itm1",
            Ast::MultItem => "mult_item",
            Ast::MultOp => "mult_op",
            Ast::SumLst1 => "sum_lst1",
            Ast::SumLst1Itm1 => "sum_lst1_itm1",
            Ast::SumItem => "sum_item",
            Ast::AddOp => "add_op",
            Ast::Plus => "plus",
            Ast::Tok(s) => s,
        };

        write!(f, "{s}")
    }

    fn emphasize(&self) -> bool {
        matches!(self, Ast::Tok(_))
    }
}

fn main() -> std::result::Result<(), anyhow::Error> {
    let tree = syntree::tree! {
        Ast::Calc => {
            Ast::CalcLst1 => {
                Ast::CalcLst1Itm1 => {
                    Ast::Instruction => {
                        Ast::Assignment => {
                            Ast::AssignItem => {
                                Ast::Id => { (Ast::Tok("c"), 1) },
                                Ast::AssignOp => { (Ast::Tok("="), 1) },
                            },
                            Ast::LocigalOr => {
                                Ast::LocigalAnd => {
                                    Ast::BitwiseOr => {
                                        Ast::BitwiseAnd => {
                                            Ast::Equality => {
                                                Ast::Relational => {
                                                    Ast::BitwiseShift => {
                                                        Ast::Sum => {
                                                            Ast::Mult => {
                                                                Ast::Power => {
                                                                    Ast::Factor => {
                                                                        Ast::Number => { (Ast::Tok("2"), 1) },
                                                                    }
                                                                },
                                                                Ast::MultLst1 => {
                                                                    Ast::MultLst1Itm1 => {
                                                                        Ast::MultItem => {
                                                                            Ast::MultOp => { (Ast::Tok("*"), 1) },
                                                                            Ast::Power => {
                                                                                Ast::Factor => {
                                                                                    Ast::Number => { (Ast::Tok("4"), 1) },
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            Ast::SumLst1 => {
                                                                Ast::SumLst1Itm1 => {
                                                                    Ast::SumItem => {
                                                                        Ast::AddOp => {
                                                                            Ast::Plus => { (Ast::Tok("+"), 1) }
                                                                        },
                                                                        Ast::Mult => {
                                                                            Ast::Power => {
                                                                                Ast::Factor => {
                                                                                    Ast::Number => { (Ast::Tok("2"), 1) },
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    },
                    (Ast::Tok(";"), 1)
                }
            }
        }
    };

    Layouter::new(&tree)
        .with_file_path("examples/example2.svg")
        .embed_with_visualize()
        .map_err(|e| anyhow::anyhow!(e))?
        .write()
        .map_err(|e| anyhow::anyhow!(e))
}
