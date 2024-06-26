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
  bar.set_style(ProgressStyle::with_template("{wide_bar:.green/blue} {eta}").unwrap().progress_chars("=>-"));

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



#[cfg(test)]
use image::io::Reader;
#[cfg(test)]
use crate::get_rays;
#[cfg(test)]
use crate::stl_parser::from_ascii;
#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Read;

//#[test]
#[cfg(test)]
#[allow(unused)]
fn test(){

  let mut buf = String::new();

  File::open("tests/teapot_ascii.stl").unwrap().read_to_string(&mut buf).unwrap();

  let bvh = BvhTree::from_mesh(from_ascii(buf),
    5, Vec3 { x: 0., y: 1.5, z: -4. });

  let rays = get_rays::<16,9>(90);

  render_and_save(bvh, rays, &"tests/teapot_ascii_test.png".to_string());

  let file_real = Reader::open("tests/teapot_ascii_real.png").unwrap().decode().unwrap();//TODO
  let file_test = Reader::open("tests/teapot_ascii_test.png").unwrap().decode().unwrap();

  assert_eq!(file_real, file_test);
}