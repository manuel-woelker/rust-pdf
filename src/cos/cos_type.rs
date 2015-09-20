use std::collections::HashMap;
use uuid::Uuid;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct IndirectCosObject {
	pub id: String,
}

#[derive(Debug, Clone)]
pub enum CosType {
	Boolean(bool),
	Integer(i64),
	Float(f64),
	String(Box<String>),
	Name(Box<String>),
	Array(Box<Vec<CosType>>),
	Dictionary(Box<HashMap<String, CosType>>),
	Stream(Box<String>),
	IndirectObject(IndirectCosObject)
}

#[derive(Debug, Clone)]
pub struct DirectCosObject {
	pub id: String,
	pub map: HashMap<String, CosType>,
	pub stream: Option<String>
}

impl DirectCosObject {
	pub fn new() -> DirectCosObject {
		DirectCosObject {id: Uuid::new_v4().to_simple_string(), map: HashMap::new(), stream: None}
	}

	pub fn new_stream(stream: String) -> DirectCosObject {
		DirectCosObject {id: Uuid::new_v4().to_simple_string(), map: HashMap::new(), stream: Some(stream)}
	}

	pub fn indirect(&self) -> CosType {
		return CosType::IndirectObject(IndirectCosObject {id: self.id.clone()});
	}

}


impl Deref for DirectCosObject {
	type Target = HashMap<String, CosType>;
    fn deref<'a>(&'a self) -> &'a HashMap<String, CosType> {
		&self.map
	}
}

impl DerefMut for DirectCosObject {
    fn deref_mut<'a>(&'a mut self) -> &'a mut HashMap<String, CosType> {
        &mut self.map
    }
}
