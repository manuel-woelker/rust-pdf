

#[derive(Debug, Clone, Default)]
pub struct PDDocumentInformation {
	pub title: Option<String>,
	pub author: Option<String>,
	pub subject: Option<String>,
	pub keywords: Option<String>,
}

impl PDDocumentInformation {
	pub fn new() -> PDDocumentInformation {
		Default::default()
	}

}
