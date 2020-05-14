use crate::ast::*;
use crate::error::Error;
use crate::parser::Parser;
use crate::tokenizer::{Token, TokenKind};

pub trait Symbol {
    fn lbp(&self) -> u32;
    fn nud(&self, parser: &mut Parser) -> Box<Node>;
    fn led(&self, parser: &mut Parser, left: Box<Node>) -> Box<Node>;
}

impl Symbol for Token {
    /// The left binding power of the token
    fn lbp(&self) -> u32 {
        use TokenKind::*;
        match &self.kind {
            End => 0,
            // Double character operators
            Range => 20,
            Assignment => 10,
            NotEqual => 40,
            GreaterEqual => 40,
            LessEqual => 40,
            DescendantWildcard => 60,
            ChainFunction => 40,
            // Named operators
            And => 30,
            Or => 25,
            In => 40,
            // Single character operators
            Period => 75,
            LeftBracket => 80,
            RightBracket => 0,
            LeftBrace => 70,
            RightBrace => 0,
            LeftParen => 80,
            RightParen => 0,
            Comma => 0,
            At => 80,
            Hash => 80,
            SemiColon => 80,
            Colon => 80,
            Question => 20,
            Add => 50,
            Sub => 50,
            Mul => 60,
            Div => 60,
            Mod => 60,
            Pipe => 20,
            Equ => 40,
            RightCaret => 40,
            LeftCaret => 40,
            Pow => 40,
            Ampersand => 50,
            Not => 0,
            Tilde => 0,
            // Literal values
            Null => 0,
            Boolean(..) => 0,
            String(..) => 0,
            Number(..) => 0,
            // Identifiers
            Name(..) => 0,
            Variable(..) => 0,
        }
    }

    /// Produce the null denotation for the token
    fn nud(&self, parser: &mut Parser) -> Box<Node> {
        use TokenKind::*;
        match &self.kind {
            Null => Box::new(Node::Null(LiteralNode {
                position: self.position,
                value: NullValue {},
            })),
            Boolean(value) => Box::new(Node::Boolean(LiteralNode {
                position: self.position,
                value: value.clone(),
            })),
            String(value) => Box::new(Node::String(LiteralNode {
                position: self.position,
                value: value.clone(),
            })),
            Number(value) => Box::new(Node::Number(LiteralNode {
                position: self.position,
                value: value.clone(),
            })),
            Name(value) => Box::new(Node::Name(LiteralNode {
                position: self.position,
                value: value.clone(),
            })),
            Variable(value) => Box::new(Node::Variable(LiteralNode {
                position: self.position,
                value: value.clone(),
            })),
            And => Box::new(Node::Name(LiteralNode {
                position: self.position,
                value: "and".to_string(),
            })),
            Or => Box::new(Node::Name(LiteralNode {
                position: self.position,
                value: "and".to_string(),
            })),
            In => Box::new(Node::Name(LiteralNode {
                position: self.position,
                value: "and".to_string(),
            })),
            Sub => Box::new(Node::UnaryMinus(UnaryNode {
                position: self.position,
                expression: parser.expression(70),
            })),
            Mul => Box::new(Node::Wildcard(BasicNode {
                position: self.position,
            })),
            DescendantWildcard => Box::new(Node::DescendantWildcard(BasicNode {
                position: self.position,
            })),
            Mod => Box::new(Node::Parent(BasicNode {
                position: self.position,
            })),
            _ => panic!(format!(
                "{:#?}",
                Error {
                    code: "S0211",
                    position: self.position,
                    message: format!("The symbol {} cannot be used as a unary operator", self)
                }
            )),
        }
    }

    /// Produce the left denotation for the token
    fn led(&self, parser: &mut Parser, left: Box<Node>) -> Box<Node> {
        use TokenKind::*;
        match &self.kind {
            Period => Box::new(Node::PathSeparator(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            Add => Box::new(Node::Add(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            Sub => Box::new(Node::Subtract(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            Mul => Box::new(Node::Multiply(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            Div => Box::new(Node::Divide(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            Mod => Box::new(Node::Modulus(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            Equ => Box::new(Node::Equal(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            LeftCaret => Box::new(Node::LessThan(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            RightCaret => Box::new(Node::GreaterThan(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            NotEqual => Box::new(Node::NotEqual(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            LessEqual => Box::new(Node::LessThanEqual(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            GreaterEqual => Box::new(Node::GreaterThanEqual(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            Ampersand => Box::new(Node::Concat(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            And => Box::new(Node::And(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            Or => Box::new(Node::Or(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            In => Box::new(Node::In(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            ChainFunction => Box::new(Node::Chain(BinaryNode {
                position: self.position,
                lhs: left,
                rhs: parser.expression(self.lbp()),
            })),
            //            TokenKind::LeftParen => {
            //                let mut arguments = Vec::new();
            //                let mut is_partial = false;
            //
            //                if parser.token().kind != TokenKind::RightParen {
            //                    loop {
            //                        match parser.token().kind {
            //                            TokenKind::Question => {
            //                                is_partial = true;
            //                                arguments.push(Node::PartialFunctionArg(BasicNode {
            //                                    position: parser.token().position,
            //                                }));
            //                                parser.expect(TokenKind::Question);
            //                            }
            //                            _ => {
            //                                arguments.push(parser.expression(0));
            //                            }
            //                        }
            //                        if parser.token().kind != TokenKind::Comma {
            //                            break;
            //                        }
            //                        parser.expect(TokenKind::Comma, false);
            //                    }
            //                }
            //                parser.expect(TokenKind::RightParen, true);
            //
            //                // TODO
            //            }
            _ => unimplemented!("led not implemented for token"),
        }
    }
}
