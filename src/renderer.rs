use crate::bvh_tree::{BvhTree, Volume};
use glam::{vec3, Vec3};
use image;
use indicatif::ProgressStyle;
use rayon::prelude::*;

pub fn render_and_save(bvh: BvhTree, rays: Vec<Vec<[f32; 3]>>, path: &str, bounces: usize, parameter: usize){
  let img = render(bvh, rays, bounces, parameter);
  img.save_with_format(path, image::ImageFormat::Png).expect("cant write picture");
}

fn render(bvh: BvhTree, rays: Vec<Vec<[f32; 3]>>, bounces: usize, parameter: usize) -> image::RgbImage{// [[px; X]; Y]

  let sun_dir = vec3(0., 1., 0.);

	let imgx = rays[0].len();
	let imgy = rays.len();

	let mut img = image::RgbImage::new( imgx as u32, imgy as u32);

  let bar = indicatif::ProgressBar::new((imgx*imgy) as u64);
  bar.set_style(ProgressStyle::with_template("{wide_bar:.green/blue} {eta}").unwrap().progress_chars("=>-"));

  //trace rays
	img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {//iter through pixels with par_iter
    bar.inc(1);

    let ray = Vec3::from_array(rays[y as usize][x as usize]).normalize();

    let traced_color: [f32; 3] = trace(&bvh.root, bvh.ambient, &ray, bounces+1, &bvh.root.camera_pos, &sun_dir, parameter);

    (*pixel).0[0] = traced_color[0] as u8;
    (*pixel).0[1] = traced_color[1] as u8;
    (*pixel).0[2] = traced_color[2] as u8;

  });//iter through pixels end
  
  bar.finish();

	img
}


//trace recusively path of ray and add with weight the resulting colors bottom up
fn trace(vol: &Volume, ambient: f32, ray: &Vec3, bounces: usize, origin: &Vec3, sun_dir: &Vec3, parameter: usize) -> [f32; 3]{
  
  //abbruchbedingung
  if bounces == 0{
    return [0f32; 3];
  }

  let (triangle, hit1) = vol.get_first_triangle_hit(ray, *origin);

  //if no hit
  if !hit1.is_finite(){
    return [0f32; 3];
  }


  let mut color_reflected = [0f32; 3];

  if triangle.reflectiveness != 0.{
    let ray_reflected = *ray - 2f32 * triangle.normal * (ray.dot(triangle.normal));

    color_reflected = trace(vol, ambient, &ray_reflected, bounces-1, &(hit1 + 1.0e-5 * triangle.normal), sun_dir, parameter);
  }

  let sun_light = hit_light(hit1, sun_dir, vol, ambient);

  match parameter {
    0 => return [ ((1f32 - triangle.reflectiveness) * triangle.color[0] + triangle.reflectiveness * color_reflected[0]) * sun_light,
    ((1f32 - triangle.reflectiveness) * triangle.color[1] + triangle.reflectiveness * color_reflected[1]) * sun_light,
    ((1f32 - triangle.reflectiveness) * triangle.color[2] + triangle.reflectiveness * color_reflected[2]) * sun_light],

    1 => [255./(origin.distance(hit1) / 2f32).exp2(); 3],

    2 => [255. * triangle.normal[0], 255. * triangle.normal[1], 255. * triangle.normal[2]],

    _ => [0., 0., 0.],//default
  }

}

//TODO: go trough light vec and summarize the different intensitys if visible from hit_point
fn hit_light(hit_point: Vec3, sun_dir: &Vec3, vol: &Volume, ambient: f32) -> f32{
  
  let _sun_pos = vec3(0.1, 100., 0.1);

  let (_, x) = vol.get_first_triangle_hit(sun_dir, hit_point);

  if x.is_finite(){
    return ambient;
  }else{
    return 1.;
  }

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

  let bvh = BvhTree::from_mesh(from_ascii(&buf, [127., 127., 255.], 0.9).unwrap(),
    5, Vec3 { x: 0., y: 1.5, z: -4. }, 0.1,);

  let rays = get_rays::<4,2>(90);

  let test = render(bvh, rays, 0, 2);

  let real = ImageBuffer::from_vec(4, 2, 
  vec![]
).unwrap();

  assert_eq!(real, test);
}
