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
		let document_information = & self.information;
		fn set_attribute(value_option: &Option<String>, name: &str, info_dictionary: &mut DirectCosObject) {
			if let Some(ref value) = *value_option {
				info_dictionary.insert(name.to_string(), CosType::String(Box::new(value.clone())));
			}
		}
		set_attribute(&document_information.title, "Title", &mut info_dictionary);
		set_attribute(&document_information.author, "Author", &mut info_dictionary);
		set_attribute(&document_information.subject, "Subject", &mut info_dictionary);
		set_attribute(&document_information.keywords, "Keywords", &mut info_dictionary);

		info_dictionary.insert("Producer".to_string(), CosType::String(Box::new(format!("rust-pdf {}", env!("CARGO_PKG_VERSION")).to_string())));

        let now = time::now_utc();
        let creation_date = time::strftime("D:%Y%m%d%H%M%SZ",&now).unwrap();
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
			document.information.author = Some("John Doe".into());
			document.information.subject = Some("The Foobar".into());
			document.information.keywords = Some("foo bar".into());


			let page_1 = PDPage::new_a4();
			let page_2 = PDPage::new_a5();

			document.add_page(page_1);
			document.add_page(page_2);

			let mut f = try!(File::create("target/test2.pdf"));
			try!(document.save(&mut f));
			Ok(())
		}
		foo().unwrap();
	}
}
