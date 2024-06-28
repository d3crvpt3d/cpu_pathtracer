use crate::stl_parser::{Mesh, Triangle};
use glam::Vec3;

#[derive(Debug)]
pub struct BvhTree{
  pub root: Box<Volume>,
  pub ambient: f32,
}

impl BvhTree{
  
  pub fn from_mesh(m: Mesh, max_elements: usize, camera_pos: Vec3, ambient_light: f32) -> Self{
    BvhTree{
      root: Box::new(Volume::new(m, max_elements, camera_pos, 0)),
      ambient: ambient_light,
    }
  }
  
}


#[allow(unused)]
#[derive(Debug)]
pub struct Volume{
  max_elements: usize,
  pub camera_pos: Vec3,
  mesh: Vec<Triangle>,
  bounding_box: (Vec3, Vec3),
  num_elements: usize,
  axis: u8,//0: x, 1: y, 2: z (mod 3)
  childs: Option<(Box<Volume>, Box<Volume>)>,
  
}

#[allow(unused)]
impl Volume{
  
  pub fn new(m: Vec<Triangle>, max_elements: usize, camera_pos: Vec3, axis: u8) -> Self{
    
    let bounding_box:(Vec3, Vec3) = Self::get_min_max(&m);
    
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
    // if vol.num_elements > max_elements{
    //   let new_axis = (axis + 1) % 3;
    //   let mesh2 = vol.split(new_axis);
      
    //   let child_a = Volume::new(vol.mesh, max_elements, camera_pos, new_axis);
    //   let child_b= Volume::new(mesh2, max_elements, camera_pos, new_axis);
      
    //   vol.mesh = Vec::new();
      
    //   vol.childs = Some((Box::new(child_a), Box::new(child_b)));
    // }
    //DEBUG
    //DEBUG
    
    vol
  }
  
  //partition triangles, modifies 'mesh' and returns new array
  pub fn split(&mut self, axis: u8) -> Vec<Triangle>{
    
    let n = self.mesh.len();
    
    //partition at n/2 by averaging current axis <=> (x0+x1+x2/3, y0+y1+y2/3, z0+z1+z2/3)
    self.mesh.select_nth_unstable_by( n/2,|e1, e2| {
      
      let uno = (e1.a + e1.b + e1.c) / 3f32;
      let dos = (e2.a + e2.b + e2.c) / 3f32;
      
      uno[axis as usize].partial_cmp(&dos[axis as usize]).expect("some float is NaN")
    });
    
    let mesh_2 = self.mesh.split_off(n/2);//[0..n/2)[n/2..len)
    
    //return childs
    mesh_2
  }
  
  pub fn get_first_triangle_hit(&self, ray: &Vec3, origin: Vec3) -> (Triangle, Vec3){//RGBA, closer AABB is the first half, because it "partitiones" it with [<,=,>]
  
    if self.childs.is_some(){//if AABB has childs test them
    
      let aabb0 = self.childs.as_ref().unwrap().0.bounding_box;
      let aabb1 = self.childs.as_ref().unwrap().1.bounding_box;
    
      let first = self.hit_box(ray);
      let second = self.hit_box(ray);
    
      let smal;
      let bigg;
    
      if first < second{
        smal = &self.childs.as_ref().unwrap().0;
        bigg = &self.childs.as_ref().unwrap().1;
      }else {
        smal = &self.childs.as_ref().unwrap().1;
        bigg = &self.childs.as_ref().unwrap().0;
      }
    
      let (triangle, depth) = smal.get_first_triangle_hit(ray, origin);
    
      if depth.is_finite(){
        return (triangle, depth);
      }else {
        return bigg.get_first_triangle_hit(ray, origin);
      }
    
    }else {//AABB is leaf
    
      let mut depth: f32 = f32::INFINITY;
      let mut out_hit_point = Vec3::INFINITY;
      
      let mut out_t: Triangle = Triangle { a: Vec3::INFINITY, b: Vec3::INFINITY, c: Vec3::INFINITY, normal: Vec3::INFINITY, reflectiveness: 0f32, color: [0.; 3]};

      for t in &self.mesh{
      
        let (triangle2, hit_point) = Volume::hit_triangle(origin, *ray, *t);
      
        let curr_depth = hit_point.distance(origin);

        if curr_depth < depth{
          depth = curr_depth;
          out_hit_point = hit_point;
          out_t = triangle2;
        }

      }
    
      return (out_t, out_hit_point);
    }
  
  }

  //https://www.jcgt.org/published/0007/03/04/paper-lowres.pdf
  pub fn hit_box(&self, ray: &Vec3) -> f32{
    let p = self.bounding_box;
    let ray_origin = self.camera_pos;
    let inv_raydir = ray.recip();

    let t0 = (p.0 - ray_origin) * inv_raydir;
    let t1 = (p.1 - ray_origin) * inv_raydir;
    let tmin = t0.min(t1);
    let tmax = t0.max(t1);
  
    if tmin.max_element() <= tmax.min_element(){
      return tmin.max_element();//TODO fix?
    }else {
      return f32::INFINITY;
    }
  }

//https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm#Rust_implementation
pub fn hit_triangle(origin: Vec3, direction: Vec3, triangle: Triangle) -> (Triangle, Vec3) {
  let e1 = triangle.b - triangle.a;
  let e2 = triangle.c - triangle.a;
  let inf3 = Vec3::INFINITY;
  let ray_cross_e2 = direction.cross(e2);
  let det = e1.dot(ray_cross_e2);
  
  if det > -f32::EPSILON && det < f32::EPSILON {
    return (triangle, inf3); // This ray is parallel to this triangle.
  }
  
  let inv_det = 1.0 / det;
  let s = origin - triangle.a;
  let u = inv_det * s.dot(ray_cross_e2);
  if u < 0.0 || u > 1.0 {
    return (triangle, inf3);
  }
  
  let s_cross_e1 = s.cross(e1);
  let v = inv_det * direction.dot(s_cross_e1);
  if v < 0.0 || u + v > 1.0 {
    return (triangle, inf3);
  }
  // At this stage we can compute t to find out where the intersection point is on the line.
  let t = inv_det * e2.dot(s_cross_e1);
  
  if t > f32::EPSILON { // ray intersection
    let intersection_point = origin + direction * t;
    return (triangle, intersection_point);
  }
  else { // This means that there is a line intersection but not a ray intersection.
    return (triangle, inf3);
  }
}

fn get_min_max(m: &Vec<Triangle>) -> (Vec3, Vec3){
  //TODO: efficient
  
  let mut minx: f32 = f32::MAX;
  let mut miny: f32 = f32::MAX;
  let mut minz: f32 = f32::MAX;
  let mut maxx: f32 = f32::MIN;
  let mut maxy: f32 = f32::MIN;
  let mut maxz: f32 = f32::MIN;
  
  for t in m {
    for vert in [t.a, t.b, t.c] {
      minx = minx.min(vert[0]);
      miny = miny.min(vert[1]);
      minz = minz.min(vert[2]);
      
      maxx = maxx.max(vert[0]);
      maxy = maxy.max(vert[1]);
      maxz = maxz.max(vert[2]);
    }
  }
  
  (Vec3::from_array([minx, miny, minz]), Vec3::from_array([maxx, maxy, maxz]))
}

}

#[test]

fn hit_box_test(){
  use crate::stl_parser::from_ascii;
  use glam::vec3;

  let str = std::fs::read_to_string("tests/pyramid_ascii.stl").unwrap();

  let tr_vec = from_ascii(str, [0.; 3], 0.);

  let vol = Volume::new(tr_vec, 10, vec3(0., 0.5, -2.), 0);

  let depth = 1.;

  assert_eq!(vol.hit_box(&vec3(0., 0., 1.)), depth)
}