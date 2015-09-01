use std::io::prelude::*;
use std::io;
use std::collections::HashMap;
use std::rc::Rc;
use std::ops::DerefMut;

pub struct CosDocument {
	pub next_object_id: u64,
	pub objects: Vec<CosObject>
}

impl CosDocument {

	pub fn new() -> CosDocument {
		CosDocument {next_object_id: 1, objects: Vec::new()}
	}

	pub fn create_object(&mut self, value: CosType) -> CosObject {
		let id = self.next_object_id;
		self.next_object_id += 1;
		let object = CosObject {id: id, generation: 0, value: value};
		object
	}

	pub fn add_object(&mut self, object: CosObject) {
		self.objects.push(object)
	}

	pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
		try!(write!(writer, "%PDF-1.1\n"));
        try!(write!(writer, "%¥±ë\n\n"));
		for object in self.objects.iter() {
			try!(write!(writer, "{} {} obj\n", object.id, object.generation));
			try!(CosDocument::write_cos_type(&object.value, writer));
			try!(write!(writer, "\nendobj\n\n"));
		}
		let trailer = r#"
trailer
  <<  /Root 1 0 R
      /Size 5
  >>
%%EOF
		"#;
		try!(write!(writer, "{}", trailer));
		Ok(())
	}

	fn write_cos_type<W: Write>(cos_type: &CosType, writer: &mut W) -> io::Result<()> {
		match *cos_type {
			CosType::Boolean(ref value) => {
				if *value {
					try!(write!(writer, "true "));
				} else {
					try!(write!(writer, "false "));
				}
			}
			CosType::Name(ref value) => {
				try!(write!(writer, "/{} ", value));
			}
			CosType::Integer(ref value) => {
				try!(write!(writer, "{} ", value));
			}
			CosType::Dictionary(ref entries) => {
				try!(write!(writer, "\n<< "));
				for (key, val) in entries.iter() {
					try!(write!(writer, "/{} ", key));
					try!(CosDocument::write_cos_type(val, writer));
				}
				try!(write!(writer, ">> "));
			}
			CosType::Array(ref values) => {
				try!(write!(writer, "[ "));
				for value in values.iter() {
					try!(CosDocument::write_cos_type(value, writer));
					try!(write!(writer, " "));
				}
				try!(write!(writer, "] "));
			}
			CosType::IndirectObject(ref object) => {
				try!(write!(writer, "{} {} R ", object.id, object.generation));
			}
			CosType::Stream(ref value) => {
				try!(write!(writer, "<< /Length {} >>\nstream\n", value.len()));
				try!(write!(writer, "{}", value));
				try!(write!(writer, "\nendstream\n"));
			}
			_ => {

			}
		}
		Ok(())
	}
}

pub struct CosObject {
	id: u64,
	generation: u64,
	value: CosType
}

impl CosObject {
	pub fn indirect(&self) -> CosType {
		return CosType::IndirectObject(IndirectCosObject {id: self.id, generation: self.generation});
	}

	pub fn get_hashmap<'a>(&'a mut self) -> &'a mut HashMap<String, CosType> {
		if let CosType::Dictionary(ref mut entries) = self.value {
			return entries.deref_mut();
		}
		panic!("Expected hashmap");
	}
}

pub struct IndirectCosObject {
	id: u64,
	generation: u64
}

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
