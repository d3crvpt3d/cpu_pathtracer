#![allow(unused)]
use std::{fmt::format, fs::{File, FileType}, io::{BufReader, Write}, sync::mpsc::channel, thread};
use indicatif::ProgressBar;
use core::time;

use bvh_tree::BvhTree;
use raycaster::ray_caster::get_rays;
//use serde::{Deserialize, Serialize};

mod raycaster;
mod renderer;
mod object_handler;
mod bvh_tree;
mod stl_parser_copy;

fn main() {

  let fov: usize = 90;
  let camera_pos: [f32; 3] = [0f32, 0f32, -6f32];
  const PIXELS: (usize, usize) = (160, 90);

  let mut args: Vec<String> = std::env::args().collect();

  //argument monads
  if args.len() < 2{
    eprintln!("3D-Object not specified, using <teapot.stl>");
    args.push("teapot.stl".to_string());
  }
  if args.len() < 3{
    eprintln!("Output-File not specified, using <traced_picture.jpg>");
    args.push("traced_picture.jpg".to_string());
  }

  let bvh: BvhTree;

  //TODO:create BVH-Tree if input file is STL not BVH
  eprintln!("Reading STL-File: {}..", &args[1]);
  let mesh = object_handler::stl_to_vec(&args[1]);

  let bvh_file = File::open(&args[1]).expect("cant open file");

  if args[1].ends_with(".stl"){

    eprintln!("Creating BVH-Tree from Mesh..");
    bvh = BvhTree::from_mesh(mesh, 4, camera_pos);//generate BVH tree

    let mut outf = File::create(format!("{}", args[1].replace("stl", "bvh"))).expect("cant create file");
    outf.write_all(serde_json::to_string_pretty(&bvh).expect("cant create string from bvh-tree").as_bytes()).expect("cant write to bvh file");

  }else if args[1].ends_with(".bvh"){

    eprintln!("Reading BVH-Tree from File..");
    bvh = serde_json::from_reader(BufReader::new(bvh_file)).unwrap();

  }else {
    eprintln!("File not 'stl' or 'bvh'");
    return;
  }

  eprintln!("Pathtracing..");
  let mut rays = get_rays::<{PIXELS.0}, {PIXELS.1}>(fov, camera_pos);

  rays = raycaster::ray_caster::transform_direction(rays);

  renderer::render_and_save(bvh, rays, &args[2]);

  println!("Done, saved to {}", args[2]);
}