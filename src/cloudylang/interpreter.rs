use super::{
    datatypes::{Dtype, Number},
    utils::{Errors, Node, Token, TokenKind},
    Parser,
};

#[derive(Debug)]
enum Rpn {
    Val(Box<dyn Dtype>),
    Op(Token),
}

fn copy_rpn(rpn: &Rpn) -> Rpn {
    match rpn {
        Rpn::Val(v) => Rpn::Val(v.copy()),
        Rpn::Op(op) => Rpn::Op(op.copy()),
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&self, input: &str, filename: &'static str) -> Result<Box<dyn Dtype>, String> {
        let ast = Parser::new().parse(input, filename)?;
        Interpreter::visit(ast)
    }

    pub fn new() -> Self {
        Interpreter {}
    }

    fn visit(node: Node) -> Result<Box<dyn Dtype>, String> {
        match node {
            Node::Number { .. } => Interpreter::visit_number_node(node),
            Node::UnaryOp { .. } => Interpreter::visit_unary_op_node(node),
            Node::Rpn { .. } => Interpreter::visit_rpn_node(node),
            _ => Err("Unsupported node type".to_string()),
        }
    }

    pub fn visit_rpn_node(node: Node) -> Result<Box<dyn Dtype>, String> {
        match node {
            Node::Rpn { rpn } => {
                let mut stack: Vec<Rpn> = Vec::new();
                for n in rpn {
                    match n {
                        Node::Op { op } => {
                            stack.push(Rpn::Op(op));
                        }
                        def => {
                            let def = Interpreter::visit(def)?;
                            stack.push(Rpn::Val(def));
                        }
                    }
                }

                let mut output: Vec<Rpn> = Vec::new();

                'main: loop {
                    while !stack.is_empty() {
                        match copy_rpn(&stack[0]) {
                            Rpn::Val(val) => {
                                stack.remove(0);
                                output.push(Rpn::Val(val.copy()));
                            }
                            Rpn::Op(op) => {
                                stack.remove(0);

                                let right = match output.pop().unwrap() {
                                    Rpn::Val(val) => val,
                                    _ => return Err("Invalid RPN expression".to_string()),
                                };
                                let left = match output.pop().unwrap() {
                                    Rpn::Val(val) => val,
                                    _ => return Err("Invalid RPN expression".to_string()),
                                };
                                let result = match op.kind {
                                    TokenKind::Plus => left.add(right)?,
                                    TokenKind::Minus => left.sub(right)?,
                                    TokenKind::Mult => left.mul(right)?,
                                    TokenKind::Div => left.div(right)?,
                                    TokenKind::Mod => left.modulo(right)?,
                                    TokenKind::Pow => left.pow(right)?,
                                    _ => {
                                        return Err(Errors::unexpected_token_error(
                                            op.copy(),
                                            "a valid operator",
                                        ))
                                    }
                                };

                                output.push(Rpn::Val(result));
                            }
                        }
                    }

                    for _i in 0..output.len() {
                        stack.push(output.pop().unwrap());
                    }

                    if stack.len() == 1 {
                        break 'main;
                    }
                }

                match stack.pop().unwrap() {
                    Rpn::Val(val) => Ok(val),
                    _ => panic!("Invalid RPN expression"),
                }
            }
            _ => panic!("Unsupported node type"),
        }
    }

    pub fn visit_number_node(node: Node) -> Result<Box<dyn Dtype>, String> {
        match node {
            Node::Number { tok } => Ok(Box::new(Number::new(tok))),
            _ => panic!("Unsupported node type!"),
        }
    }

    pub fn visit_unary_op_node(node: Node) -> Result<Box<dyn Dtype>, String> {
        match node {
            Node::UnaryOp { op, right } => {
                let right = Interpreter::visit(*right)?;

                match op.kind {
                    TokenKind::Minus => right.neg(),
                    TokenKind::Plus => right.pos(),
                    _ => panic!("Unsupported token type!"),
                }
            }
            _ => panic!("Unsupported node type!"),
        }
    }
}
