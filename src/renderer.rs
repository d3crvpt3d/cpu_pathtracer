pub struct ImgBuffer<const X: usize, const Y: usize>{
	pub array: [[(u8, u8, u8); Y]; X], //RGB
}

impl<const X: usize, const Y: usize> ImgBuffer<X, Y>{
	pub fn new() -> Self{
		ImgBuffer::<X, Y>{
			array: [[(0u8, 0u8, 0u8); Y]; X],
		}
	}
}