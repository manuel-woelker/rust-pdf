#[derive(Debug, Clone)]
pub struct PDRectangle {
	pub lower_left_x: f32,
	pub lower_left_y: f32,
	pub upper_right_x: f32,
	pub upper_right_y: f32,
}

impl PDRectangle {
	pub fn new(lower_left_x: f32, lower_left_y: f32, upper_right_x: f32, upper_right_y: f32) -> PDRectangle {
		PDRectangle {
			lower_left_x: lower_left_x,
			lower_left_y: lower_left_y,
			upper_right_x: upper_right_x,
			upper_right_y: upper_right_y
		}
	}

}
