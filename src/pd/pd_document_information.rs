

#[derive(Debug, Clone, Default)]
pub struct PDDocumentInformation {
	pub title: Option<String>
}

impl PDDocumentInformation {
	pub fn new() -> PDDocumentInformation {
		Default::default()
	}

}
