use bvh_tree::BvhTree;
use glam::Vec3;
use raycaster::ray_caster::get_rays;

mod raycaster;
mod renderer;
mod object_handler;
mod bvh_tree;
mod stl_parser;

fn main() -> std::io::Result<()>{
  
  //settings
  let fov: usize = 90;
  let camera_pos: Vec3 = Vec3::from_array([-0.5, 1., -4.]);
  let bounces = 2;
  let max_elements = 20;
  let color = [255., 32., 255.];
  let reflectiveness = 0.2;
  let ambient_light = 0.1;
  const PIXELS: (usize, usize) = (1920, 1080);
  //settings
  

  let mut args: Vec<String> = std::env::args().collect();
  
  //argument monads
  if args.len() < 2{
    eprintln!("3D-Object not specified, using <storage/teapot.stl>");
    args.push("storage/teapot.stl".to_string());
  }
  if args.len() < 3{
    eprintln!("Output-File not specified, using <storage/traced_picture.png>");
    args.push("storage/traced_picture.png".to_string());
  }
  
  println!("STL-File: {}", &args[1]);
  let mesh = object_handler::stl_to_vec(&args[1], color, reflectiveness)?;
  
  println!("Creating BVH-Tree from Mesh");
  let bvh = BvhTree::from_mesh(mesh, max_elements, camera_pos, ambient_light);//generate BVH tree
  
  println!("Pathtracing");
  let mut rays = get_rays::<{PIXELS.0}, {PIXELS.1}>(fov);
  
  rays = raycaster::ray_caster::transform_direction(rays);//TODO
  
  renderer::render_and_save(bvh, rays, &args[2], bounces);
  
  println!("Done, saved to {}", args[2]);
  Ok(())
}