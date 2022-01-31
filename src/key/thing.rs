use crate::err::Error;
use crate::key::bytes::{deserialize, serialize};
use crate::key::BASE;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Thing {
	kv: String,
	_a: String,
	ns: String,
	_b: String,
	db: String,
	_c: String,
	tb: String,
	_d: String,
	id: String,
}

impl Into<Vec<u8>> for Thing {
	fn into(self) -> Vec<u8> {
		self.encode().unwrap()
	}
}

impl From<Vec<u8>> for Thing {
	fn from(val: Vec<u8>) -> Self {
		Thing::decode(&val).unwrap()
	}
}

pub fn new(ns: &str, db: &str, tb: &str, id: &str) -> Thing {
	Thing::new(ns.to_string(), db.to_string(), tb.to_string(), id.to_string())
}

impl Thing {
	pub fn new(ns: String, db: String, tb: String, id: String) -> Thing {
		Thing {
			kv: BASE.to_owned(),
			_a: String::from("*"),
			ns,
			_b: String::from("*"),
			db,
			_c: String::from("*"),
			tb,
			_d: String::from("*"),
			id,
		}
	}
	pub fn encode(&self) -> Result<Vec<u8>, Error> {
		Ok(serialize(self)?)
	}
	pub fn decode(v: &[u8]) -> Result<Thing, Error> {
		Ok(deserialize(v)?)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn key() {
		use super::*;
		#[rustfmt::skip]
		let val = Thing::new(
			"test".to_string(),
			"test".to_string(),
			"test".to_string(),
			"test".into(),
		);
		let enc = Thing::encode(&val).unwrap();
		let dec = Thing::decode(&enc).unwrap();
		assert_eq!(val, dec);
	}
}
