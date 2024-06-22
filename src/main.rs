#![allow(unused)]
use std::{sync::mpsc::channel, thread::{self}, fmt::format, fs::File, io::{BufReader, Write}};
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
  let camera_pos: [f32; 3] = [0f32; 3];
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

  //create BVH-Tree if input file is STL not BVH
  //if args[1].ends_with("stl"){

    eprintln!("Reading STL-File: {}..", &args[1]);
    let mesh = object_handler::stl_to_vec(&args[1]);

    eprintln!("Creating BVH-Tree from Mesh..");
    bvh = BvhTree::from_mesh(mesh, 4, camera_pos);//generate BVH tree
    
    //let mut f = File::create(format!("{}.bvh",&args[1][0..args[1].len()-4])).unwrap(); //open output file from "original.stl" to "original.bvh"
    //f.write_all(serde_json::to_string(&bvh).unwrap().as_bytes());//save bvh file

  //}else {

    //eprintln!("Reading BVH-Tree from {}..", &args[1]);
    //bvh = serde_json::from_reader(BufReader::new(File::open(&args[1]).unwrap())).unwrap();//read BVH-Tree from File
  
  //}

  eprintln!("Pathtracing..");
  let rays = get_rays::<{PIXELS.0}, {PIXELS.1}>(fov, camera_pos);

  renderer::render_and_save(bvh, rays, &args[2]);

  println!("Done, saved to {}", args[2]);
}