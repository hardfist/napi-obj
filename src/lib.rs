#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use serde::{Serialize, Deserialize};


#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
#[napi(object)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AddNode{
  pub left: String,
  pub right: String,
}

#[napi(object)]
#[derive(Debug,Serialize, Deserialize)]
pub struct MultNode {
  pub left: String,
  pub right: String
}
#[napi(object)]
#[derive(Debug,Serialize, Deserialize)]
pub struct AST {
  pub left: AddNode,
  pub right: MultNode
}
#[napi]
pub fn print_as_obj() -> AST {
  return AST {
    left: AddNode { left: "1".to_string(), right: "2".to_string() },
    right: MultNode { left: "1".to_string(), right: "2".to_string() }
  }
}

#[napi]
pub fn print_as_string() -> String {
  let ast =  AST {
    left: AddNode { left: "1".to_string(), right: "2".to_string() },
    right: MultNode { left: "1".to_string(), right: "2".to_string() }
  };
  let str = serde_json::to_string(&ast).unwrap();
  str
}