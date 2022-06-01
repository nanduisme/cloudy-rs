use std::fmt::Debug;

use crate::cloudylang::utils::Position;

pub trait Dtype: Debug {
    fn add(&self, other: Box<dyn Dtype>) -> Result<Box<dyn Dtype>, String>;
    fn sub(&self, other: Box<dyn Dtype>) -> Result<Box<dyn Dtype>, String>;
    fn mul(&self, other: Box<dyn Dtype>) -> Result<Box<dyn Dtype>, String>;
    fn div(&self, other: Box<dyn Dtype>) -> Result<Box<dyn Dtype>, String>;
    fn modulo(&self, other: Box<dyn Dtype>) -> Result<Box<dyn Dtype>, String>;
    fn pow(&self, other: Box<dyn Dtype>) -> Result<Box<dyn Dtype>, String>;

    fn neg(&self) -> Result<Box<dyn Dtype>, String>;
    fn pos(&self) -> Result<Box<dyn Dtype>, String>;

    fn copy(&self) -> Box<dyn Dtype>;
    fn dbg(&self) -> String;
    fn disp(&self) -> String;
    fn get_value(&self) -> f64;
    fn get_name(&self) -> &'static str;
    fn get_pos(&self) -> Position;
}
