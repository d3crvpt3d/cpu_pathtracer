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
    let ambient = bvh.ambient;

    let mut ray1 = Vec3::from_array(rays[y as usize][x as usize]);

    for bounce in 0..(bounces+1){
      let box_depth = groot.hit_box(&ray1);
		
      //if ray hit box
		  if box_depth.is_finite(){

        //get intersection point + triangle
        let (triangle, hit_point) = groot.get_first_triangle_hit(&ray1, groot.camera_pos);

        //test if hit_point is in sunlight
        let (_, hit_point2) = groot.get_first_triangle_hit(&vec3(0.5, 1., -0.5), hit_point+(3f32*f32::EPSILON*triangle.normal));//3 epsylon above
        

        //ambient light * color, if point is in shade
        if hit_point2.is_finite(){

          (*pixel).0[0] = (*pixel).0[0].overflowing_add((triangle.color[0] * ambient * triangle.falloff.powi(bounce as i32)) as u8).0;
          (*pixel).0[1] = (*pixel).0[1].overflowing_add((triangle.color[1] * ambient * triangle.falloff.powi(bounce as i32)) as u8).0;
          (*pixel).0[2] = (*pixel).0[2].overflowing_add((triangle.color[2] * ambient * triangle.falloff.powi(bounce as i32)) as u8).0;
        
        }else{//bright light * color, if point is in sunlight; with falloff: '(triangle.color[0] * triangle.falloff.powi(bounce as i32) / (box_depth*box_depth)) as u8'

          (*pixel).0[0] = (*pixel).0[0].overflowing_add((triangle.color[0] * triangle.falloff.powi(bounce as i32)) as u8).0;
          (*pixel).0[1] = (*pixel).0[1].overflowing_add((triangle.color[1] * triangle.falloff.powi(bounce as i32)) as u8).0;
          (*pixel).0[2] = (*pixel).0[2].overflowing_add((triangle.color[2] * triangle.falloff.powi(bounce as i32)) as u8).0;

        }

        //cast boncing ray //math.stackexchange.com/questions/13261/how-to-get-a-reflection-vector + a bit seperation
        let new_ray = (ray1-2f32*(ray1.dot(triangle.normal))*triangle.normal) + (3f32*f32::EPSILON*triangle.normal);
			  
        ray1 = new_ray;
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
