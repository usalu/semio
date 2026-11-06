//! 📖️ Native pixel-law binaries share immutable fixture reads from their declared product root.
use std::{collections::HashMap,sync::{Mutex,OnceLock}};
fn leaked(relative:&str)->&'static[u8]{
 static READ:OnceLock<Mutex<HashMap<String,&'static[u8]>>>=OnceLock::new();
 let mut read=READ.get_or_init(Default::default).lock().unwrap();
 read.entry(relative.to_owned()).or_insert_with(||{let path=crate::product_directory().join(relative);Box::leak(std::fs::read(&path).unwrap_or_else(|error|panic!("test input {}: {error}",path.display())).into_boxed_slice())})
}
pub fn text(relative:&str)->&'static str{std::str::from_utf8(leaked(relative)).unwrap_or_else(|error|panic!("test input {relative} is not UTF-8: {error}"))}
pub fn bytes(relative:&str)->&'static[u8]{leaked(relative)}
