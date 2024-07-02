use std::{fs::File, io::{self, Read}};
//use stl_parser::{Mesh, Triangle, Vertex}; copied from this
use crate::stl_parser::{self, Mesh};

pub fn stl_to_vec(path: &str, color: [f32; 3], reflectiveness: f32) -> io::Result<Mesh>{
  
  let mut buf = [0u8; 5];

  let mut file = File::open(path).expect("cant open STL-File");

  file.read(&mut buf)?;

  let contents = String::from_utf8(buf.to_vec()).unwrap();

  //return vector either with ascii or binary parser
  if contents.as_str() == "solid"{

    println!("Reading ASCII");

    stl_parser::from_ascii(path, color, reflectiveness)
  
  }else{

    println!("Reading Binary");

    stl_parser::from_binary(path, color, reflectiveness)

  }
  
}