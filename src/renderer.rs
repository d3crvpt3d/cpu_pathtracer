use crate::bvh_tree::BvhTree;
use glam::{vec3, Vec3};
use image;
use indicatif::ProgressStyle;
use rayon::prelude::*;

pub fn render_and_save(bvh: BvhTree, rays: Vec<Vec<[f32; 3]>>, path: &String, bounces: usize){
  let img = render(bvh, rays, bounces);
  img.save_with_format(path, image::ImageFormat::Png).expect("cant write picture");
}

fn render(bvh: BvhTree, rays: Vec<Vec<[f32; 3]>>, bounces: usize) -> image::RgbImage{// [[px; X]; Y]

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

    for _ in 0..(bounces+1){
      let mut k = groot.hit_box(&ray);

      if k.is_finite(){//if hit get closest triangle intersection
        k = groot.get_first_hit_depth(&ray, groot.camera_pos);
      };
		
		  if k.is_finite(){
        let origin = groot.camera_pos + (ray.normalize()*k) + vec3(0., f32::EPSILON*32., 0.);//origin is epsylon over hitted point
        if groot.get_first_hit_depth(&vec3(0., 1., 0.), origin).is_infinite(){
          (*pixel).0[0] = (*pixel).0[0].overflowing_add((255f32 * (10f32/k)) as u8).0;
          (*pixel).0[1] = (*pixel).0[1].overflowing_add((255f32 * (10f32/k)) as u8).0;//sun area
          (*pixel).0[2] = (*pixel).0[2].overflowing_add((255f32 * (10f32/k)) as u8).0;
        }else{
          (*pixel).0[0] = (*pixel).0[0].overflowing_add((32f32 * (10f32/k)) as u8).0;
          (*pixel).0[1] = (*pixel).0[1].overflowing_add((32f32 * (10f32/k)) as u8).0;//shady area
          (*pixel).0[2] = (*pixel).0[2].overflowing_add((32f32 * (10f32/k)) as u8).0;
        }
			  
		  }else {
			  *pixel = no_color;
		  }
    }

  });
  
  bar.finish();
	
	img
}


//#[test] //TODO
#[cfg(test)]
#[allow(unused)]
fn test(){
  use image::ImageBuffer;
  use std::{fs::File, io::Read};
  use crate::stl_parser::from_ascii;
  use crate::raycaster::ray_caster::get_rays;


  let mut buf = String::new();

  File::open("tests/teapot_ascii.stl").unwrap().read_to_string(&mut buf).unwrap();

  let bvh = BvhTree::from_mesh(from_ascii(buf),
    5, Vec3 { x: 0., y: 1.5, z: -4. });

  let rays = get_rays::<4,2>(90);

  let test = render(bvh, rays, 0);

  let real = ImageBuffer::from_vec(4, 2, 
  vec![]
).unwrap();

  assert_eq!(real, test);
}