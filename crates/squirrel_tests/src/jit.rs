use std::{collections::HashMap, mem, sync::Arc};

use miette::NamedSource;
use squirrel_ast::{
    atom::BinOp,
    expr::Expression,
    stmt::{Block, Statement},
};
use squirrel_jit::{CodeGenerator, Signature, Typ};
use squirrel_lex::token::Span;

#[test]
pub fn test_jit_1() {
    let source = Arc::new(NamedSource::new("-".to_string(), "-".to_string()));
    let mut generator = CodeGenerator::new().unwrap();
    let result = generator.codegen(
        Signature::new(
            "sum",
            HashMap::from([("a".to_string(), Typ::Int), ("b".to_string(), Typ::Int)]),
            Some(Typ::Int),
        ),
        &Block {
            span: Span(source.clone(), 0..0),
            statements: vec![Statement::Return {
                span: Span(source.clone(), 0..0),
                expr: Some(Expression::Bin {
                    span: Span(source.clone(), 0..0),
                    op: BinOp::Add,
                    lhs: Box::new(Expression::Variable {
                        span: Span(source.clone(), 0..0),
                        name: "a".to_string(),
                    }),
                    rhs: Box::new(Expression::Variable {
                        span: Span(source.clone(), 0..0),
                        name: "b".to_string(),
                    }),
                }),
            }],
        },
    );
    match result {
        Ok(result) => {
            let code_fn = unsafe { mem::transmute::<_, fn(i64, i64) -> i64>(result) };
            println!("res: {}", code_fn(5, 15));
        }
        Err(_) => {}
    }
    println!("{result:?}")
}
