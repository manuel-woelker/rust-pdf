extern crate uuid;

pub mod cos;



#[cfg(test)]
mod tests {
    use cos::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io;
    use std::mem;
    use std::rc::Rc;
    use std::collections::HashMap;

#[test]
fn write_pdf() {
    println!("Sizeof Rc: {}", mem::size_of::<Rc<CosType>>());
    println!("Sizeof CosType: {}", mem::size_of::<CosType>());
    println!("Sizeof HashMap: {}", mem::size_of::<HashMap<String, String>>());
    println!("Sizeof String: {}", mem::size_of::<String>());
    println!("Sizeof Box: {}", mem::size_of::<Box<String>>());
    fn foo() -> io::Result<()> {
        let mut f = try!(File::create("target/test.pdf"));
        let mut document = CosDocument::new();

        let mut catalog = DirectCosObject::new();
        catalog.insert("Type".to_string(), CosType::Name(Box::new("Catalog".to_string())));


        let mut pages = DirectCosObject::new();
        pages.insert("Type".to_string(), CosType::Name(Box::new("Pages".to_string())));
        pages.insert("Count".to_string(), CosType::Integer(1));
        pages.insert("MediaBox".to_string(), CosType::Array(Box::new(vec![
            CosType::Integer(0), CosType::Integer(0), CosType::Integer(300), CosType::Integer(144)])));
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

        document.add_object(catalog);
        document.add_object(pages);
        document.add_object(page);
        document.add_object(stream);

        try!(document.write(&mut f));
        Ok(())
    }
    foo().unwrap();
}

}
