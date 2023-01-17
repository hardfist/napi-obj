#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
#[napi(object)]
#[derive(Debug)]
pub struct User {
  pub name: String,
  pub age: u32
}
#[napi]
pub fn print(user:User){
  println!("user: {:?}",user);
}