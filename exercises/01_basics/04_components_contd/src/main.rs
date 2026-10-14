// This component should:
// 1. read a single string argument from the command line
// 2. parse that input into a set of tokens as youve done before (reuse your code!)
// 2. Evaluate these tokens according to the operator precedence rules (parentheses first, multiple/divide next, the plus/minus)
// 3. Print out the result to stdout
// If you get stuck, ask a neighbor or grab a trainer!

use functions::{Token, tokenize};

pub fn main() {
    let expr = std::env::args().next().expect("No value from CLI input");
    println!("{}", evaluate(tokenize(&expr)));
}

/// An operator waiting on the operator stack.
#[derive(Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    /// Unary minus, as in `-5`.
    Neg,
    /// Not an operator, but a barrier that `)` unwinds the stack to.
    LeftParen,
}

impl Op {
    /// Higher binds tighter. `(` is lowest so no operator ever pops it.
    fn precedence(self) -> u8 {
        match self {
            Op::LeftParen => 0,
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div => 2,
            Op::Neg => 3,
        }
    }
}

/// Evaluates the tokens in a single loop with the shunting-yard algorithm.
///
/// Numbers go onto `values`. Operators wait on `ops` until an operator with lower or equal
/// precedence, a `)` or the end of input shows their operands are complete, and are then applied.
fn evaluate(tokens: impl IntoIterator<Item = Token>) -> f64 {
    let mut values: Vec<f64> = Vec::new();
    let mut ops: Vec<Op> = Vec::new();
    // True where a number, `(` or unary `-` may come next, false where a binary operator or `)` may.
    let mut expect_operand = true;

    for token in tokens {
        match token {
            Token::Number(n) if expect_operand => {
                values.push(n);
                expect_operand = false;
            }
            Token::LeftParen if expect_operand => ops.push(Op::LeftParen),
            Token::Minus if expect_operand => ops.push(Op::Neg),
            Token::RightParen if !expect_operand => loop {
                match ops.pop() {
                    Some(Op::LeftParen) => break,
                    Some(op) => apply(op, &mut values),
                    None => panic!("unmatched `)`"),
                }
            },
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide if !expect_operand => {
                let op = match token {
                    Token::Plus => Op::Add,
                    Token::Minus => Op::Sub,
                    Token::Multiply => Op::Mul,
                    _ => Op::Div,
                };
                // `>=` applies equal precedence first, so `10 - 4 - 3` is `(10 - 4) - 3`.
                while let Some(&top) = ops.last()
                    && top.precedence() >= op.precedence()
                {
                    ops.pop();
                    apply(top, &mut values);
                }
                ops.push(op);
                expect_operand = true;
            }
            token => panic!("unexpected token {token:?}"),
        }
    }

    if expect_operand {
        panic!("expression ended where a number was expected");
    }
    while let Some(op) = ops.pop() {
        match op {
            Op::LeftParen => panic!("unmatched `(`"),
            op => apply(op, &mut values),
        }
    }
    values.pop().expect("a complete expression leaves exactly one value")
}

/// Pops `op`'s operands off `values` and pushes the result.
fn apply(op: Op, values: &mut Vec<f64>) {
    // `expect_operand` in `evaluate` guarantees every operator has its operands.
    let rhs = values.pop().expect("operator has a right operand");
    let result = match op {
        Op::Neg => -rhs,
        Op::LeftParen => unreachable!("`(` is never applied"),
        op => {
            let lhs = values.pop().expect("binary operator has a left operand");
            match op {
                Op::Add => lhs + rhs,
                Op::Sub => lhs - rhs,
                Op::Mul => lhs * rhs,
                _ => lhs / rhs,
            }
        }
    };
    values.push(result);
}
