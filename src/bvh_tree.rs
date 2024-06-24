#![allow(unused)]
use std::{cmp::{max, min, Ordering}, f32::NAN};

use crate::stl_parser_copy::{Mesh, Triangle};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct BvhTree{
  pub root: Box<Volume>,
  pub camera_pos: [f32; 3],
}

impl BvhTree{
  
  pub fn from_mesh(m: Mesh, max_elements: usize, camera_pos: [f32; 3]) -> Self{
    BvhTree{
      root: Box::new(Volume::new(m.triangles, max_elements, camera_pos)),
      camera_pos,
    }
  }

}

//credit zacharmarz
fn hit_box(ray: &[f32; 3], vol: &Volume) -> (bool, f32){

  let teil_x: f32 = 1. /ray[0];
  let teil_y: f32 = 1. /ray[1];
  let teil_z: f32 = 1. /ray[2];

  let bb_lw_xy = vol.bounding_box.0;
  let bb_gr_xy = vol.bounding_box.1;

  let cam_pos = vol.camera_pos;

  let t1: f32 = (bb_lw_xy[0] - cam_pos[0]) * teil_x;
  let t2: f32 = (bb_gr_xy[0] - cam_pos[0]) * teil_x;
  let t3: f32 = (bb_lw_xy[1] - cam_pos[1]) * teil_y;
  let t4: f32 = (bb_gr_xy[1] - cam_pos[1]) * teil_y;
  let t5: f32 = (bb_lw_xy[2] - cam_pos[2]) * teil_z;
  let t6: f32 = (bb_gr_xy[2] - cam_pos[2]) * teil_z;

  let tmin = f32::max(f32::max(f32::min(t1, t2), f32::min(t1, t2)), f32::min(t5, t6));
  let tmax = f32::min(f32::min(f32::max(t1, t2), f32::max(t1, t2)), f32::max(t5, t6));

  let t: f32;

  if tmax < 0f32{
    t = tmax;
    return (false, t);
  }

  if tmin > tmax{
    t = tmax;
    return (false, t);
  }

  //intersects
  t = tmin;
  return (true, t);
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Volume{
  max_elements: usize,
  camera_pos: [f32; 3],
  mesh: Vec<Triangle>,
  bounding_box: ([f32; 3], [f32; 3]),
  num_elements: usize,
  axis: u8,//0: x, 1: y, 2: z (mod 3)
  childs: Option<(Box<Volume>, Box<Volume>)>,

}

#[allow(unused)]
impl Volume{
  
  pub fn next_axis(&self) -> u8{
    (self.axis + 1) % 3
  }

  pub fn new(m: Vec<Triangle>, max_elements: usize, camera_pos: [f32; 3]) -> Self{
    
    let bounding_box:([f32; 3], [f32; 3]) = Self::get_min_max(&m);

    let mut vol = Volume{
      max_elements,
      camera_pos,
      num_elements: m.len(),
      mesh: m,//Vec<Triangles>
      bounding_box,
      axis: 0,
      childs: None,
    };

    //DEBUG
    //DEBUG
    if vol.num_elements > max_elements{
      let mesh2 = vol.split(max_elements,(vol.axis + 1) % 3);

      let child_a = Volume::new(vol.mesh, max_elements, camera_pos);
      let child_b= Volume::new(mesh2, max_elements, camera_pos);

      vol.mesh = Vec::new();

      vol.childs = Some((Box::new(child_a), Box::new(child_b)));
    }
    //DEBUG
    //DEBUG

    vol
  }

  //partition triangles, modifies 'mesh' and returns new array
  pub fn split(&mut self, max_elements: usize, axis: u8) -> Vec<Triangle>{

    let n = self.mesh.len();

    //partition at n/2
    self.mesh.select_nth_unstable_by( n/2,|e1, e2| {
      e1.vertices[0][axis as usize].partial_cmp(&e2.vertices[0][axis as usize]).expect("some float is NaN")
    });
    
    let mesh_2 = self.mesh.split_off(n/2);//[0..n/2)[n/2..len)

    //return childs
    mesh_2
  }

  pub fn get_first_hit_color(&self, ray: &[f32; 3]) -> Option<[u8; 3]>{//RGBA, closer AABB is the first half, because it "partitiones" it with [<,=,>]

    let (hit, depth) = hit_box(ray, self);

    let depth_with_falloff: [u8; 3];

    if hit{
      
      if self.childs.is_some(){//if AABB has childs test them first
        
        let inner = self.childs.as_ref().unwrap().0.get_first_hit_color(ray);

        if inner.is_some(){
          return inner;
        }else {
          return self.childs.as_ref().unwrap().1.get_first_hit_color(ray);
        }

      }else {//AABB is leaf
        
        for t in &self.mesh{

          let mut depth: f32 = f32::INFINITY;

          let curr_depth = self.hit_triangle(ray, t);

          if curr_depth.is_some(){
            depth = depth.min(Self::distance(&self.camera_pos, &curr_depth.unwrap()));
          }

          return Some([(255f32 / depth) as u8; 3]);//depth := color
        }
        
        depth_with_falloff = [(255f32 / depth) as u8; 3];//calculate percieved depth
        return Some(depth_with_falloff);//DEBUG, real value: Some([0x00u8; 3])
      }

    }
    None
  }

  //pythagoras
  fn distance(a: &[f32; 3], b: &[f32; 3]) -> f32{
    f32::sqrt((b[0]-a[0]).powi(2) + (b[1]-a[1]).powi(2) + (b[2]-a[2]).powi(2))
  }

  //returns intersection point
  pub fn hit_triangle(&self, ray: &[f32; 3], t: &Triangle) -> Option<[f32; 3]>{//https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm

    let epsylon = f32::EPSILON;

    let edge1 = Self::sub(&t.vertices[1], &t.vertices[0]);
    let edge2 = Self::sub(&t.vertices[2], &t.vertices[0]);
    
    let ray_cross_e2 = Self::cross(ray, &edge2);
    let det = Self::dot(&edge1, &ray_cross_e2);

    if det > -epsylon && det < epsylon{
      return None;
    }

    let inv_det = 1. / det;
    let s = Self::sub(&self.camera_pos, &t.vertices[0]);
    let u = inv_det * Self::dot(ray, &ray_cross_e2);

    if u > 0. || u > 1. {
      return None;
    }

    let s_cross_e1 = Self::cross(&s, &edge1);
    let v = inv_det * Self::dot(ray, &s_cross_e1);

    if v < 0. || u + v > 1.{
      return None;
    }

    let t = inv_det * Self::dot(ray, &s_cross_e1);

    if t > epsylon{
      return Some(Self::add(&self.camera_pos, &Self::mul(ray, t)));
    }else {
      return None;
    }

  }

  fn mul(a: &[f32; 3], b: f32) -> [f32; 3]{
    [a[0]*b, a[1]*b, a[2]*b]
  }

  fn add(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3]{
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
  }

  fn dot(a: &[f32; 3], b: &[f32; 3]) -> f32{
    a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
  }
  
  fn cross(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3]{
    [a[1]*b[2] - a[2]*b[1], a[2]*b[0] - a[0]*b[2], a[0]*b[1] - a[1]*b[0]]
  }

  fn sub(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3]{
    [ a[0] - b[0], a[1] - b[1], a[2] - b[2]]
  }

  fn get_min_max(m: &Vec<Triangle>) -> ([f32; 3], [f32; 3]){
    //TODO: efficient

    let mut minx: f32 = f32::MAX;
    let mut miny: f32 = f32::MAX;
    let mut minz: f32 = f32::MAX;
    let mut maxx: f32 = f32::MIN;
    let mut maxy: f32 = f32::MIN;
    let mut maxz: f32 = f32::MIN;

    for t in m {
      for vert in t.vertices {
        for point in vert {

          minx = minx.min(point);
          miny = miny.min(point);
          minz = minz.min(point);

          maxx = maxx.max(point);
          maxy = maxy.max(point);
          maxz = maxz.max(point);

        }
      }
    }

    ([minx, miny, minz], [maxx, maxy, maxz])
  }

}