#![allow(unused)]
use std::{fmt::format, fs::{File, FileType}, io::{BufReader, Write}, sync::mpsc::channel, thread};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use core::time;

use bvh_tree::BvhTree;
use raycaster::ray_caster::get_rays;
use serde_json;

mod raycaster;
mod renderer;
mod object_handler;
mod bvh_tree;
mod stl_parser_copy;

fn main() {

  let fov: usize = 90;
  let camera_pos: [f32; 3] = [0., 1., -4.];
  const PIXELS: (usize, usize) = (800, 450);

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

  let bvh: BvhTree;

  let in_file = File::open(&args[1]).expect("cant open file");

  if args[1].ends_with(".stl"){

    eprintln!("Reading STL-File: {}", &args[1]);
    let mesh = object_handler::stl_to_vec(&args[1]);

    let mut out_mesh = File::create(format!("{}", args[1].replace("stl", "msh"))).expect("cant create 'msh' file");
    out_mesh.write_all(serde_json::to_string_pretty(&mesh).expect("cant create string from 'mesh'").as_bytes()).expect("cant write to 'mesh' file");

    eprintln!("Creating BVH-Tree from Mesh");
    bvh = BvhTree::from_mesh(mesh, 4, camera_pos);//generate BVH tree

    let mut out_bvh = File::create(format!("{}", args[1].replace("stl", "bvh"))).expect("cant create 'bvh' file");
    out_bvh.write_all(serde_json::to_string_pretty(&bvh).expect("cant create string from bvh-tree").as_bytes()).expect("cant write to 'bvh' file");
    

  }else if args[1].ends_with(".bvh"){

    eprintln!("Reading BVH-Tree from File");
    bvh = serde_json::from_reader(BufReader::new(in_file)).unwrap();

  }else if args[1].ends_with(".msh"){

    let mesh = serde_json::from_reader(BufReader::new(in_file)).unwrap();

    eprintln!("Creating BVH-Tree from Mesh");
    bvh = BvhTree::from_mesh(mesh, 4, camera_pos);//generate BVH tree

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