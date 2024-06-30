use std::f32::EPSILON;

use crate::bvh_tree::{BvhTree, Volume};
use glam::{vec3, Vec3};
use image;
use indicatif::ProgressStyle;
use rayon::prelude::*;

pub fn render_and_save(bvh: BvhTree, rays: Vec<Vec<[f32; 3]>>, path: &String, bounces: usize){
  let img = render(bvh, rays, bounces);
  img.save_with_format(path, image::ImageFormat::Png).expect("cant write picture");
}

fn render(bvh: BvhTree, rays: Vec<Vec<[f32; 3]>>, bounces: usize) -> image::RgbImage{// [[px; X]; Y]

  let sun_dir = vec3(0., 1., 0.);

	let imgx = rays[0].len();
	let imgy = rays.len();

	let mut img = image::RgbImage::new( imgx as u32, imgy as u32);

  let bar = indicatif::ProgressBar::new((imgx*imgy) as u64);
  bar.set_style(ProgressStyle::with_template("{wide_bar:.green/blue} {eta}").unwrap().progress_chars("=>-"));

  //trace rays
	img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {//iter through pixels
    bar.inc(1);

    let ray = Vec3::from_array(rays[y as usize][x as usize]);

    let traced_color: [f32; 3] = trace(&bvh.root, bvh.ambient, &ray, bounces+1, &bvh.root.camera_pos, &sun_dir);

    (*pixel).0[0] = traced_color[0] as u8;
    (*pixel).0[1] = traced_color[1] as u8;
    (*pixel).0[2] = traced_color[2] as u8;

  });//iter through pixels end
  
  bar.finish();
	
	img
}


//trace recusively path of ray and add with weight the resulting colors bottom up
fn trace(vol: &Volume, ambient: f32, ray: &Vec3, bounces: usize, origin: &Vec3, sun_dir: &Vec3) -> [f32; 3]{

  //abbruchbedingung
  if bounces == 0{
    return [0f32; 3];
  }

  let (triangle, hit1) = vol.get_first_triangle_hit(ray, *origin);

  //if no hit
  if !hit1.is_finite(){
    return [0f32; 3];
  }

  let ray_reflected = *ray - 2f32 * triangle.normal * (ray.dot(triangle.normal));

  let color_reflected = trace(vol, ambient, &ray_reflected, bounces-1, &(hit1 + EPSILON * triangle.normal), sun_dir);

  let col: [f32; 3];

  if hit_light(ray, sun_dir, vol) == 0.{
    col = [
      (1f32 - triangle.reflectiveness) * triangle.color[0] + triangle.reflectiveness * color_reflected[0] * ambient,
      (1f32 - triangle.reflectiveness) * triangle.color[1] + triangle.reflectiveness * color_reflected[1] * ambient,
      (1f32 - triangle.reflectiveness) * triangle.color[2] + triangle.reflectiveness * color_reflected[2] * ambient
    ];
  }else{//multiply by light strength
    col = [
      (1f32 - triangle.reflectiveness) * triangle.color[0] + triangle.reflectiveness * color_reflected[0],
      (1f32 - triangle.reflectiveness) * triangle.color[1] + triangle.reflectiveness * color_reflected[1],
      (1f32 - triangle.reflectiveness) * triangle.color[2] + triangle.reflectiveness * color_reflected[2]
    ];
  }

  return col;
}

//TODO: go trough light vec and summarize the different intensitys if visible from hit_point
fn hit_light(ray: &Vec3, sun_dir: &Vec3, vol: &Volume) -> f32{
  return 1.;
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

  File::open("tests/pyramid_ascii.stl").unwrap().read_to_string(&mut buf).unwrap();

  let bvh = BvhTree::from_mesh(from_ascii(buf, [127., 127., 255.], 0.9),
    5, Vec3 { x: 0., y: 1.5, z: -4. }, 0.1,);

  let rays = get_rays::<4,2>(90);

  let test = render(bvh, rays, 0);

  let real = ImageBuffer::from_vec(4, 2, 
  vec![]
).unwrap();

  assert_eq!(real, test);
}
