use super::Dtype;
use crate::cloudylang::utils::{Errors, Position, Token, TokenKind};

#[derive(Clone, Debug)]
pub struct Number {
    value: f64,
    position: Position,
}

impl Number {
    pub fn new(tok: Token) -> Number {
        match tok.kind {
            TokenKind::Number(v) => Number {
                value: v,
                position: tok.pos.copy(),
            },
            _ => panic!("Expected number but found {}", tok.name()),
        }
    }

    pub fn new_from_value(value: f64, pos: Position) -> Number {
        Number {
            value,
            position: pos,
        }
    }
}

impl Dtype for Number {
    fn get_value(&self) -> f64 {
        self.value
    }

    fn get_name(&self) -> &'static str {
        "number"
    }

    fn get_pos(&self) -> Position {
        self.position.copy()
    }

    fn copy(&self) -> Box<dyn Dtype> {
        Box::new(Number::new_from_value(self.value, self.position.copy()))
    }

    fn dbg(&self) -> String {
        // Display the value without decimals if it doesnt make a difference to the actual value
        if self.value.fract() == 0.0 {
            format!("{}", self.value.trunc())
        } else {
            format!("{}", self.value)
        }
    }

    fn disp(&self) -> String {
        self.dbg()
    }

    fn add(&self, other: Box<dyn Dtype>) -> Result<Box<dyn Dtype>, String> {
        if other.get_name() != "number" {
            return Err(Errors::type_error(
                other.get_pos(),
                "number",
                other.get_name(),
            ));
        }

        let other = other.get_value();
        Ok(Box::new(Number::new_from_value(
            self.value + other,
            self.get_pos(),
        )))
    }

    fn sub(&self, other: Box<dyn Dtype>) -> Result<Box<dyn Dtype>, String> {
        if other.get_name() != "number" {
            return Err(Errors::type_error(
                other.get_pos(),
                "number",
                other.get_name(),
            ));
        }

        let other = other.get_value();
        Ok(Box::new(Number::new_from_value(
            self.value + -other,
            self.get_pos(),
        )))
    }

    fn mul(&self, other: Box<dyn Dtype>) -> Result<Box<dyn Dtype>, String> {
        if other.get_name() != "number" {
            return Err(Errors::type_error(
                other.get_pos(),
                "number",
                other.get_name(),
            ));
        }

        let other = other.get_value();
        Ok(Box::new(Number::new_from_value(
            self.value * other,
            self.get_pos(),
        )))
    }

    fn div(&self, other: Box<dyn Dtype>) -> Result<Box<dyn Dtype>, String> {
        if other.get_name() != "number" {
            return Err(Errors::type_error(
                other.get_pos(),
                "number",
                other.get_name(),
            ));
        }

        let other_val = other.get_value();
        if other_val == 0.0 {
            Err(Errors::div_by_zero_error(other.get_pos()))
        } else {
            Ok(Box::new(Number::new_from_value(
                self.value / other_val,
                self.get_pos(),
            )))
        }
    }

    fn modulo(&self, other: Box<dyn Dtype>) -> Result<Box<dyn Dtype>, String> {
        if other.get_name() != "number" {
            return Err(Errors::type_error(
                other.get_pos(),
                "number",
                other.get_name(),
            ));
        }

        let other_val = other.get_value();
        if other_val == 0.0 {
            Err(Errors::div_by_zero_error(other.get_pos()))
        } else {
            Ok(Box::new(Number::new_from_value(
                self.value % other_val,
                self.get_pos(),
            )))
        }
    }

    fn pow(&self, other: Box<dyn Dtype>) -> Result<Box<dyn Dtype>, String> {
        if other.get_name() != "number" {
            return Err(Errors::type_error(
                other.get_pos(),
                "number",
                other.get_name(),
            ));
        }

        let other_val = other.get_value();
        Ok(Box::new(Number::new_from_value(
            self.value.powf(other_val),
            self.get_pos(),
        )))
    }

    fn neg(&self) -> Result<Box<dyn Dtype>, String> {
        if self.value == 0.0 {
            Ok(Box::new(Number::new_from_value(0.0, self.get_pos())))
        } else {
            Ok(Box::new(Number::new_from_value(
                -self.value,
                self.get_pos(),
            )))
        }
    }

    fn pos(&self) -> Result<Box<dyn Dtype>, String> {
        Ok(Box::new(Number::new_from_value(self.value, self.get_pos())))
    }
}
