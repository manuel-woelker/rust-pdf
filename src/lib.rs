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

        let mut foo = document.create_direct_object();
        foo.borrow_mut().insert("Type".to_string(), CosType::Name(Box::new("Catalog".to_string())));
/*
        try!(write!(f, "%PDF-1.1\n"));
        try!(write!(f, "%¥±ë\n\n"));*/
        let mut catalog_hashmap = HashMap::new();
        catalog_hashmap.insert("Type".to_string(), CosType::Name(Box::new("Catalog".to_string())));
        let catalog = CosType::Dictionary(Box::new(catalog_hashmap));
        let mut catalog_object = document.create_object(catalog);


        let mut pages_hashmap = Box::new(HashMap::new());
        pages_hashmap.insert("Type".to_string(), CosType::Name(Box::new("Pages".to_string())));
        pages_hashmap.insert("Count".to_string(), CosType::Integer(1));
        pages_hashmap.insert("MediaBox".to_string(), CosType::Array(Box::new(vec![
            CosType::Integer(0), CosType::Integer(0), CosType::Integer(300), CosType::Integer(144)])));
        let pages = CosType::Dictionary(pages_hashmap);
        let mut pages_object = document.create_object(pages);
        catalog_object.get_hashmap().insert("Pages".to_string(), pages_object.indirect());

        let mut page_hashmap: HashMap<String, CosType> = HashMap::new();
        page_hashmap.insert("Type".to_string(), CosType::Name(Box::new("Page".to_string())));

        let mut f1_hashmap = HashMap::new();
        f1_hashmap.insert("Type".to_string(), CosType::Name(Box::new("Font".to_string())));
        f1_hashmap.insert("Subtype".to_string(), CosType::Name(Box::new("Type1".to_string())));
        f1_hashmap.insert("BaseFont".to_string(), CosType::Name(Box::new("Times-Roman".to_string())));
        let mut font_hashmap = HashMap::new();
        font_hashmap.insert("F1".to_string(),  CosType::Dictionary(Box::new(f1_hashmap)));
        let mut resource_hashmap = HashMap::new();
        resource_hashmap.insert("Font".to_string(),  CosType::Dictionary(Box::new(font_hashmap)));
        page_hashmap.insert("Resources".to_string(),  CosType::Dictionary(Box::new(resource_hashmap)));

        let page = CosType::Dictionary(Box::new(page_hashmap));
        let mut page_object = document.create_object(page);
        pages_object.get_hashmap().insert("Kids".to_string(), CosType::Array(Box::new(vec![page_object.indirect()])));
        page_object.get_hashmap().insert("Parent".to_string(), pages_object.indirect());

        let stream = CosType::Stream(Box::new(
r#"  BT
    /F1 18 Tf
    0 0 Td
    (Hello World) Tj
  ET"#.to_string()));
        let stream_object = document.create_object(stream);
        page_object.get_hashmap().insert("Contents".to_string(), stream_object.indirect());

        document.add_object(catalog_object);
        document.add_object(pages_object);
        document.add_object(page_object);
        document.add_object(stream_object);

        try!(document.write(&mut f));
        Ok(())
    }
    foo().unwrap();
}

}
