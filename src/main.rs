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
  let camera_pos: Vec3 = Vec3::from_array([0., 0., -2.]);
  let bounces = 2;
  let max_elements = 20;
  let color = [128., 128., 128.];
  let reflectiveness = 0.2;
  let ambient_light = 0.1;
  const PIXELS: (usize, usize) = (16000, 9000);
  //settings
  

  let mut args: Vec<String> = std::env::args().collect();

  let mut arg_idx = 1;

  for a in &args{
    if a.starts_with("-"){
      arg_idx += 1;
    }
  }
  
  
  //argument monads
  if args.len() < arg_idx + 1{
    eprintln!("3D-Object not specified, using <storage/teapot.stl>");
    args.push("storage/teapot.stl".to_string());
  }
  if args.len() < arg_idx + 2{
    eprintln!("Output-File not specified, using <storage/traced_picture.png>");
    args.push("storage/traced_picture.png".to_string());
  }

  let mut parameter = 0;

  //test for -d {depth-file}
  for arg in &args{
    let arg_c: Vec<char> = arg.chars().collect();
    if arg_c[0] == '-'{
      if arg_c.get(1).is_none(){eprintln!("use a letter after -");return Ok(());}
      parameter = match arg_c[1] {
        'd' => 1,
        'n' => 2,
        _ => 0,
      }
    }
  }

  let in_path = args[arg_idx].as_str();
  let out_path = args[arg_idx+1].as_str();

  println!("STL-File: {}", in_path);
  let mesh = match parameter {
    0 => object_handler::stl_to_vec(in_path, color, reflectiveness)?,
    1 => object_handler::stl_to_vec(in_path, [0., 0., 0.], 0.)?,
    2 => object_handler::stl_to_vec(in_path, [0., 0., 0.], 0.)?,
    _ => object_handler::stl_to_vec(in_path, color, reflectiveness)?,
  };
    
  println!("Creating BVH-Tree from Mesh");
  let bvh = BvhTree::from_mesh(mesh, max_elements, camera_pos, ambient_light);//generate BVH tree

  println!("Generating Rays");
  let mut rays = get_rays::<{PIXELS.0}, {PIXELS.1}>(fov);

  rays = raycaster::ray_caster::transform_direction(rays);//TODO

  println!("Pathtracing");
  match parameter {
    0 => renderer::render_and_save(bvh, rays, out_path, bounces, parameter),
    1 => renderer::render_and_save(bvh, rays, out_path, 0, parameter),
    2 => renderer::render_and_save(bvh, rays, out_path, 0, parameter),
    _ => eprintln!("wrong format for parameter"),
  }

  println!("Done, saved to {}", out_path);
  Ok(())
}
