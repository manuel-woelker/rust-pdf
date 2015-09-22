use std::io::prelude::*;
use std::io::{self};
use pd::{PDDocumentInformation, PDPage};
use cos::{CosDocument, DirectCosObject, CosType};
use time;

#[derive(Debug, Clone)]
pub struct PDDocument {
	pub information: PDDocumentInformation,
	pub pages: Vec<PDPage>,
}

impl PDDocument {
	pub fn new() -> PDDocument {
		PDDocument {information: PDDocumentInformation::new(), pages: Vec::new()}
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

		let mut pages = DirectCosObject::new();
		pages.insert("Type".to_string(), CosType::Name(Box::new("Pages".to_string())));
		pages.insert("Count".to_string(), CosType::Integer(self.pages.len() as i64));
		catalog.insert("Pages".to_string(), pages.indirect());

		let mut cos_pages = Vec::new();

		for page in self.pages.iter() {
			let mut cos_page = DirectCosObject::new();
	        cos_page.insert("Type".to_string(), CosType::Name(Box::new("Page".to_string())));
			let media_box = & page.media_box;
			cos_page.insert("MediaBox".to_string(), CosType::Array(Box::new(vec![
				CosType::Float(media_box.lower_left_x as f64),
				CosType::Float(media_box.lower_left_y as f64),
				CosType::Float(media_box.upper_right_x as f64),
				CosType::Float(media_box.upper_right_y as f64)])));
			cos_page.insert("Parent".to_string(), pages.indirect());
			cos_pages.push(cos_page);
		}

		cos_document.add_object(catalog);
        cos_document.add_object(info_dictionary);

		let mut pages_kids = Vec::new();

		for cos_page in cos_pages {
			pages_kids.push(cos_page.indirect());
			cos_document.add_object(cos_page);
		}

		pages.insert("Kids".to_string(), CosType::Array(Box::new(pages_kids)));
		cos_document.add_object(pages);


		try!(cos_document.write(writer));
		Ok(())
	}

	pub fn add_page(&mut self, page: PDPage) {
		self.pages.push(page);
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

			let page_1 = PDPage::new_a4();
			let page_2 = PDPage::new_a5();

			document.add_page(page_1);
			document.add_page(page_2);

			println!("Document: {:?}", document);
			let mut f = try!(File::create("target/test2.pdf"));
			try!(document.save(&mut f));
			Ok(())
		}
		foo().unwrap();
	}
}
