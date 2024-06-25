use crate::bvh_tree::BvhTree;
use glam::Vec3;
use image;
use indicatif::ProgressStyle;
use rayon::prelude::*;

pub fn render_and_save(bvh: BvhTree, rays: Vec<Vec<[f32; 3]>>, path: &String){// [[px; X]; Y]

	let imgx = rays[0].len();
	let imgy = rays.len();

	let no_color = image::Rgb([0x00u8; 3]);
		
	let mut img = image::RgbImage::new( imgx as u32, imgy as u32);

  let bar = indicatif::ProgressBar::new((imgx*imgy) as u64);
  bar.set_style(ProgressStyle::with_template("{wide_bar:.green/blue} {eta}").unwrap().progress_chars("#>-"));

	img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {

    bar.inc(1);

    let groot = &bvh.root;

    let ray = Vec3::from_array(rays[y as usize][x as usize]);

    let mut k = groot.hit_box(&ray);

    if k.is_finite(){//if hit get closest triangle intersection
      k = groot.get_first_hit_depth(&ray);
    };
		
		if k.is_finite(){
			*pixel = image::Rgb([(255f32 / k) as u8; 3]);
		}else {
			*pixel = no_color;
		}

  });
  
  bar.finish();
	
	img.save_with_format(path, image::ImageFormat::Png).expect("cant write picture");
}