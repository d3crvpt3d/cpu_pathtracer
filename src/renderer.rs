use crate::bvh_tree::BvhTree;
use image;

pub fn render_and_save(bvh: BvhTree, rays: Vec<Vec<[f32; 3]>>, path: &String){// [[px; X]; Y]

	let imgx = rays[0].len();
	let imgy = rays.len();
		
	let mut img = image::RgbImage::new( imgx as u32, imgy as u32);

	for (x, y, pixel) in img.enumerate_pixels_mut(){
		let color = match bvh.root.get_first_hit_color(&rays[y as usize][x as usize]) {
				Some(c) 	=> image::Rgb(c),
				_ 								=> image::Rgb([0x00u8, 0x00u8, 0x00u8]),
		};
		*pixel = color;
	}
	
	img.save(path).expect("cant write picture");
}