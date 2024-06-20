use std::{fs::File, io::Read};
use stl_parser::*;

pub fn read_to_polygon_vec(path: &str) -> Mesh{
  
  let mut buf: String = String::new();

  File::open(path).expect("cant open STL-File").read_to_string(&mut buf).unwrap();

  Mesh::from_ascii(buf)

}