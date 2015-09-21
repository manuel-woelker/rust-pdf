use std::io::prelude::*;
use std::io::{self};
use pd::PDDocumentInformation;
use cos::{CosDocument, DirectCosObject, CosType};
use time;

#[derive(Debug, Clone)]
pub struct PDDocument {
	pub information: PDDocumentInformation
}

impl PDDocument {
	pub fn new() -> PDDocument {
		PDDocument {information: PDDocumentInformation::new()}
	}

	pub fn save<W: Write + Seek>(& self, writer: &mut W) -> io::Result<()> {
		let mut cos_document = CosDocument::new();

		let mut catalog = DirectCosObject::new();
        catalog.insert("Type".to_string(), CosType::Name(Box::new("Catalog".to_string())));

		let mut info_dictionary = DirectCosObject::new();
        info_dictionary.insert("Title".to_string(), CosType::String(Box::new("rust-pdf test document".to_string())));
        let now = time::now_utc();
        let creation_date = time::strftime("D:%Y%m%d%H%M%SZ",&now).unwrap();
        println!("CreationDate: {}", creation_date);
        info_dictionary.insert("CreationDate".to_string(), CosType::String(Box::new(creation_date.clone())));
        info_dictionary.insert("ModDate".to_string(), CosType::String(Box::new(creation_date)));
        cos_document.add_object(catalog);
        cos_document.add_object(info_dictionary);
		try!(cos_document.write(writer));
		Ok(())
	}
}


#[cfg(test)]
mod tests {
    use pd::*;
    use std::fs::File;
    use std::io;


	#[test]
	fn write_pdf() {
	    fn foo() -> io::Result<()> {
			let mut document = PDDocument::new();
			document.information.title = Some("Foobar".into());
			println!("Document: {:?}", document);
			let mut f = try!(File::create("target/test2.pdf"));
			try!(document.save(&mut f));
			Ok(())
		}
		foo().unwrap();
	}
}
