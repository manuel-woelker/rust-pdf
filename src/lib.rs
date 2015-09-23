extern crate uuid;
extern crate time;


pub mod cos;
pub mod pd;



#[cfg(test)]
mod tests {
    use cos::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io;
    use std::collections::HashMap;
    use time;

#[test]
fn write_pdf() {
    fn foo() -> io::Result<()> {
        let mut f = try!(File::create("target/test.pdf"));
        let mut document = CosDocument::new();

        let mut catalog = DirectCosObject::new();
        catalog.insert("Type".to_string(), CosType::Name(Box::new("Catalog".to_string())));


        let mut pages = DirectCosObject::new();
        pages.insert("Type".to_string(), CosType::Name(Box::new("Pages".to_string())));
        pages.insert("Count".to_string(), CosType::Integer(1));
        pages.insert("MediaBox".to_string(), CosType::Array(Box::new(vec![
            CosType::Integer(0), CosType::Integer(0), CosType::Float(595.276), CosType::Float(841.89)])));
        catalog.insert("Pages".to_string(), pages.indirect());

        let mut page = DirectCosObject::new();
        page.insert("Type".to_string(), CosType::Name(Box::new("Page".to_string())));

        let mut f1_hashmap = HashMap::new();
        f1_hashmap.insert("Type".to_string(), CosType::Name(Box::new("Font".to_string())));
        f1_hashmap.insert("Subtype".to_string(), CosType::Name(Box::new("Type1".to_string())));
        f1_hashmap.insert("BaseFont".to_string(), CosType::Name(Box::new("Times-Roman".to_string())));
        let mut font_hashmap = HashMap::new();
        font_hashmap.insert("F1".to_string(),  CosType::Dictionary(Box::new(f1_hashmap)));
        let mut resource_hashmap = HashMap::new();
        resource_hashmap.insert("Font".to_string(),  CosType::Dictionary(Box::new(font_hashmap)));
        page.insert("Resources".to_string(),  CosType::Dictionary(Box::new(resource_hashmap)));

        pages.insert("Kids".to_string(), CosType::Array(Box::new(vec![page.indirect()])));
        page.insert("Parent".to_string(), pages.indirect());

        let stream = DirectCosObject::new_stream(
r#"  BT
    /F1 18 Tf
    0 0 Td
    (Hello World) Tj
  ET"#.to_string());
        page.insert("Contents".to_string(), stream.indirect());

        let mut info_dictionary = DirectCosObject::new();
        info_dictionary.insert("Title".to_string(), CosType::String(Box::new("rust-pdf test document".to_string())));
        info_dictionary.insert("Author".to_string(), CosType::String(Box::new("Manuel Woelker".to_string())));
        info_dictionary.insert("Producer".to_string(), CosType::String(Box::new(format!("rust-pdf {}", env!("CARGO_PKG_VERSION")).to_string())));
        let now = time::now_utc();
        let creation_date = time::strftime("D:%Y%m%d%H%M%SZ",&now).unwrap();
        info_dictionary.insert("CreationDate".to_string(), CosType::String(Box::new(creation_date.clone())));
        info_dictionary.insert("ModDate".to_string(), CosType::String(Box::new(creation_date)));
        document.add_object(catalog);
        document.add_object(info_dictionary);
        document.add_object(pages);
        document.add_object(page);
        document.add_object(stream);

        try!(document.write(&mut f));
        Ok(())
    }
    foo().unwrap();
}

}
