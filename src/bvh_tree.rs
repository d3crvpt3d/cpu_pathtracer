use crate::stl_parser_copy::{Mesh, Triangle};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct BvhTree{
  pub root: Box<Volume>,
}

impl BvhTree{
  
  pub fn from_mesh(m: Mesh, max_elements: usize, camera_pos: [f32; 3]) -> Self{
    BvhTree{
      root: Box::new(Volume::new(m.triangles, max_elements, camera_pos, 0))
    }
  }

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

  pub fn new(m: Vec<Triangle>, max_elements: usize, camera_pos: [f32; 3], axis: u8) -> Self{
    
    let bounding_box:([f32; 3], [f32; 3]) = Self::get_min_max(&m);

    let mut vol = Volume{
      max_elements,
      camera_pos,
      num_elements: m.len(),
      mesh: m,//Vec<Triangles>
      bounding_box,
      axis,
      childs: None,
    };

    //DEBUG
    //DEBUG
    if vol.num_elements > max_elements{
      let new_axis = (axis + 1) % 3;
      let mesh2 = vol.split(new_axis);

      let child_a = Volume::new(vol.mesh, max_elements, camera_pos, new_axis);
      let child_b= Volume::new(mesh2, max_elements, camera_pos, new_axis);

      vol.mesh = Vec::new();

      vol.childs = Some((Box::new(child_a), Box::new(child_b)));
    }
    //DEBUG
    //DEBUG

    vol
  }

  //partition triangles, modifies 'mesh' and returns new array
  pub fn split(&mut self, axis: u8) -> Vec<Triangle>{

    let n = self.mesh.len();

    //partition at n/2 by averaging current axis <=> (x0+x1+x2/3, y0+y1+y2/3, z0+z1+z2/3)
    self.mesh.select_nth_unstable_by( n/2,|e1, e2| {

      let uno = e1.vertices[axis as usize];
      let dos = e2.vertices[axis as usize];

      let avg1 = uno[0] + uno[1] + uno[2] / 3.;
      let avg2 = dos[0] + dos[1] + dos[2] / 3.;

      avg1.partial_cmp(&avg2).expect("some float is NaN")
    });
    
    let mesh_2 = self.mesh.split_off(n/2);//[0..n/2)[n/2..len)

    //return childs
    mesh_2
  }

  pub fn get_first_hit_depth(&self, ray: &[f32; 3]) -> f32{//RGBA, closer AABB is the first half, because it "partitiones" it with [<,=,>]

    if self.childs.is_some(){//if AABB has childs test them
        
      let first = self.childs.as_ref().unwrap().0.hit_box(ray);
      let second = self.childs.as_ref().unwrap().1.hit_box(ray);

      let smal;
      let bigg;

      if first < second{
        smal = &self.childs.as_ref().unwrap().0;
        bigg = &self.childs.as_ref().unwrap().1;
      }else {
        smal = &self.childs.as_ref().unwrap().1;
        bigg = &self.childs.as_ref().unwrap().0;
      }

      let depth = smal.get_first_hit_depth(ray);

      if depth.is_finite(){
        return depth;
      }else {
        return bigg.get_first_hit_depth(ray);
      }

    }else {//AABB is leaf
    
      let mut depth: f32 = f32::INFINITY;

      for t in &self.mesh{

        let curr_depth = Self::distance(&self.hit_triangle(ray, t).unwrap_or(
                                  [f32::INFINITY, f32::INFINITY, f32::INFINITY]
                                )
                              );

        depth = depth.min(curr_depth);
      }

      return depth;
    }

  }

  //pythagoras
  fn distance(a: &[f32; 3]) -> f32{
    f32::sqrt((a[0]).powi(2) + (a[1]).powi(2) + (a[2]).powi(2))
  }

  //fast reciprocal
  fn rcp(a: &[f32; 3]) -> [f32; 3]{
    [a[0].recip() , a[1].recip(), a[2].recip()]
  }

  fn min3(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3]{
    if Self::abs3(a) < Self::abs3(b){
      return *a;
    }else {
      return *b;
    }
  }

  fn max3(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3]{
    if Self::abs3(a) > Self::abs3(b){
      return *a;
    }else {
      return *b;
    }
  }

  fn abs3(a: &[f32; 3]) -> f32{
    f32::sqrt(a[0].powi(2) + a[1].powi(2) + a[2].powi(2))
  }

  fn matmul(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3]{//maybe
    [a[0] * b[0], a[1] * b[1], a[2] * b[2]]
  }

  //credit zacharmarz
  pub fn hit_box(&self, ray: &[f32; 3]) -> f32{

    let origin = self.camera_pos;
    let aabb = self.bounding_box;

    let inv_d = Self::rcp(ray);
    let t0s = Self::matmul(&Self::sub3(&aabb.0, &origin), &inv_d);
    let t1s = Self::matmul(&Self::sub3(&aabb.1, &origin), &inv_d);

    let tsmaller = Self::min3(&t0s, &t1s);
    let tbigger = Self::max3(&t0s, &t1s);


    let tmin = f32::max(f32::max(tsmaller[0], tsmaller[1]), tsmaller[2]);
    let tmax = f32::min(f32::min(tbigger[0], tbigger[1]), tbigger[2]);

    if tmin < tmax{
      return tmin;//yes hit
    }
    return f32::INFINITY;//no hit
  }

  //returns intersection point
  pub fn hit_triangle(&self, direction: &[f32; 3], t: &Triangle) -> Option<[f32; 3]>{//https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm

    let origin = self.camera_pos;
    let epsylon = f32::EPSILON;

    let edge1 = Self::sub3(&t.vertices[1], &t.vertices[0]);
    let edge2 = Self::sub3(&t.vertices[2], &t.vertices[0]);
    
    let ray_cross_e2 = Self::cross(direction, &edge2);
    let det = Self::dot(&edge1, &ray_cross_e2);

    if det > -epsylon && det < epsylon{
      return None;
    }

    let inv_det = 1. / det;
    let s = Self::sub3(&origin, &t.vertices[0]);
    let u = inv_det * Self::dot(&s, &ray_cross_e2);

    if u < 0. || u > 1. {
      return None;
    }

    let s_cross_e1 = Self::cross(&s, &edge1);
    let v = inv_det * Self::dot(direction, &s_cross_e1);

    if v < 0. || u + v > 1.{
      return None;
    }

    let t = inv_det * Self::dot(&edge2, &s_cross_e1);

    if t > epsylon{
      return Some(Self::add(&origin, &Self::mul(direction, t)));
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

  fn sub3(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3]{
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
          minx = minx.min(vert[0]);
          miny = miny.min(vert[1]);
          minz = minz.min(vert[2]);

          maxx = maxx.max(vert[0]);
          maxy = maxy.max(vert[1]);
          maxz = maxz.max(vert[2]);
      }
    }

    ([minx, miny, minz], [maxx, maxy, maxz])
  }

}