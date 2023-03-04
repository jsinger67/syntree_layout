use syntree_layout::{Layouter, Visualize};

#[derive(Debug)]
struct MyNodeData(i32);

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
    fn visualize(&self) -> String {
        match self {
            Ast::Calc => "calc".to_string(),
            Ast::CalcLst1 => "calc_lst1".to_string(),
            Ast::CalcLst1Itm1 => "calc_lst1_itm1".to_string(),
            Ast::Instruction => "instruction".to_string(),
            Ast::Assignment => "assignment".to_string(),
            Ast::AssignItem => "assign_item".to_string(),
            Ast::Id => "id".to_string(),
            Ast::AssignOp => "assign_op".to_string(),
            Ast::LocigalOr => "locigal_or".to_string(),
            Ast::LocigalAnd => "locigal_and".to_string(),
            Ast::BitwiseOr => "bitwise_or".to_string(),
            Ast::BitwiseAnd => "bitwise_and".to_string(),
            Ast::Equality => "equality".to_string(),
            Ast::Relational => "relational".to_string(),
            Ast::BitwiseShift => "bitwise_shift".to_string(),
            Ast::Sum => "sum".to_string(),
            Ast::Mult => "mult".to_string(),
            Ast::Power => "power".to_string(),
            Ast::Factor => "factor".to_string(),
            Ast::Number => "number".to_string(),
            Ast::MultLst1 => "mult_lst1".to_string(),
            Ast::MultLst1Itm1 => "mult_lst1_itm1".to_string(),
            Ast::MultItem => "mult_item".to_string(),
            Ast::MultOp => "mult_op".to_string(),
            Ast::SumLst1 => "sum_lst1".to_string(),
            Ast::SumLst1Itm1 => "sum_lst1_itm1".to_string(),
            Ast::SumItem => "sum_item".to_string(),
            Ast::AddOp => "add_op".to_string(),
            Ast::Plus => "plus".to_string(),
            Ast::Tok(s) => s.to_string(),
        }
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
