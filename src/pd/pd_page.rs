use pd::PDRectangle;

#[derive(Debug, Clone)]
pub struct PDPage {
	pub media_box: PDRectangle
}

const DEFAULT_USER_SPACE_UNIT_DPI: f32 = 72.0;
const MM_TO_UNITS: f32 = 1.0/(10.0*2.54)*DEFAULT_USER_SPACE_UNIT_DPI;

impl PDPage {
		pub fn new(media_box: PDRectangle) -> PDPage {
			PDPage {media_box: media_box}
		}

		pub fn new_a4() -> PDPage {
			Self::new(PDRectangle::new(0.0, 0.0, 210.0*MM_TO_UNITS, 297.0*MM_TO_UNITS))
		}

		pub fn new_a5() -> PDPage {
			Self::new(PDRectangle::new(0.0, 0.0, 148.0*MM_TO_UNITS, 210.0*MM_TO_UNITS))
		}
}
