use std::io::prelude::*;
use std::io::{self, SeekFrom};
use std::collections::HashMap;

use cos::*;

pub struct CosDocument {
	pub objects: Vec<DirectCosObject>,
}

impl CosDocument {

	pub fn new() -> CosDocument {
		CosDocument {objects: Vec::new()}
	}

	pub fn add_object(&mut self, object: DirectCosObject) {
		self.objects.push(object)
	}

	pub fn write<W: Write + Seek>(& self, writer: &mut W) -> io::Result<()> {
		try!(CosWriter::write(self, writer));
		Ok(())
	}

}





struct CosWriter {
	id_map: HashMap<String, u64>
}

impl CosWriter {
	pub fn write<W: Write + Seek>(document: &CosDocument, writer: &mut W) -> io::Result<()> {
		let mut cos_writer = CosWriter {id_map: HashMap::new()};

		try!(write!(writer, "%PDF-1.1\n"));
		try!(write!(writer, "%¥±ë\n\n"));
		// assign ids
		let mut next_object_id = 1;
		for object in document.objects.iter() {
			cos_writer.id_map.insert(object.id.clone(), next_object_id);
			next_object_id += 1;
		}
		let objects = document.objects.clone();
		for object in objects.iter() {
			let offset = try!(writer.seek(SeekFrom::Current(0)));
			println!("Offset: {}", offset);
			try!(write!(writer, "{} 0 obj\n", cos_writer.id_map.get(&object.id).unwrap()));
			if object.stream.is_some() {
				let stream = object.stream.as_ref().unwrap();
				try!(cos_writer.write_stream(stream, writer));
			} else {
				try!(cos_writer.write_dictionary(&object.map, writer));
			}
			try!(write!(writer, "\nendobj\n\n"));
		}
		let offset = try!(writer.seek(SeekFrom::Current(0)));
		println!("Offset: {}", offset);
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

	fn write_dictionary<W: Write>(&mut self, dictionary: & HashMap<String, CosType>, writer: &mut W) -> io::Result<()> {
		try!(write!(writer, "\n<< "));
		for (key, val) in dictionary.iter() {
			try!(write!(writer, "/{} ", key));
			try!(self.write_cos_type(val, writer));
		}
		try!(write!(writer, ">> "));
		Ok(())
	}

	fn write_stream<W: Write>(&mut self, stream: & String, writer: &mut W) -> io::Result<()> {
		try!(write!(writer, "<< /Length {} >>\nstream\n", stream.len()));
		try!(write!(writer, "{}", stream));
		try!(write!(writer, "\nendstream\n"));
		Ok(())
	}

	fn write_cos_type<W: Write>(&mut self, cos_type: &CosType, writer: &mut W) -> io::Result<()> {
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
					try!(self.write_cos_type(val, writer));
				}
				try!(write!(writer, ">> "));
			}
			CosType::Array(ref values) => {
				try!(write!(writer, "[ "));
				for value in values.iter() {
					try!(self.write_cos_type(value, writer));
					try!(write!(writer, " "));
				}
				try!(write!(writer, "] "));
			}
			CosType::IndirectObject(ref object) => {
				try!(write!(writer, "{} 0 R ", self.id_map.get(&object.id).unwrap()));
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
