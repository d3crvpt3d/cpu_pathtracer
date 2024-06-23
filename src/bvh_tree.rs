#![allow(unused)]
use crate::stl_parser_copy::{Mesh, Triangle};
//use serde::{Serialize, Deserialize};

//#[derive(Serialize, Deserialize)]
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

fn hit_box(ray: &[f32; 3], vol: &Volume) -> bool{
  todo!("calc vector/box intersection");
}

fn hit_triangle(ray: &[f32; 3], t: &Triangle) -> bool{
  todo!("calc vector/triangle intersection");
}

#[allow(unused)]
//#[derive(Serialize, Deserialize)]
pub struct Volume{
  max_elements: usize,
  camera_pos: [f32; 3],
  //#[serde(with = "MeshDef")]
  mesh: Vec<Triangle>,
  bounding_box: ((f32, f32), (f32, f32)),
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
    let bounding_box:((f32, f32), (f32, f32)) = Volume::get_min_max(&m);
    Volume{
      max_elements,
      camera_pos,
      num_elements: m.len(),
      mesh: m,//Vec<Triangles>
      bounding_box,
      axis: 0,
      childs: None,
    }
  }

  //partition triangles and give [0..len/2] = child.0, [len/2..len-1] = child.1
  pub fn split(&mut self, max_elements: usize, axis: u8) -> Option<(Box<Volume>,Box<Volume>)>{

    let n = self.num_elements;
    
    let mut mesh_1: Vec<Triangle> = Vec::new();
    let mut mesh_2: Vec<Triangle> = Vec::new();

    let mut median: f32 = 0f32;

    self.mesh.iter().for_each(|e| {
      median += e.vertices[0][axis as usize];
    });

    //partition at n/2
    self.mesh.select_nth_unstable_by( n/2,|e1, e2| {
      e1.vertices[0][axis as usize].partial_cmp(&e2.vertices[0][axis as usize]).expect("some float is NaN")
    });

    mesh_1 = self.mesh[0..n/2].to_vec();//[0..n/2]
    mesh_2 = self.mesh[n/2..n].to_vec();//[n/2..n]

    let mut vol1 = Volume::new(mesh_1, max_elements, self.camera_pos);
    let mut vol2 = Volume::new(mesh_2, max_elements, self.camera_pos);


    let nxt_axis: u8 = (self.axis + 1) % 3;//increment axis for children

    //recursively split childs if elements > max_elements
    if vol1.mesh.len() > vol1.max_elements{
      vol1.split(max_elements, nxt_axis);
    }

    if vol2.mesh.len() > vol2.max_elements{
      vol2.split(max_elements, nxt_axis);
    }

    //return childs
    Some(
      (
        Box::new(vol1),
        Box::new(vol2)
      )
    )
  }

  pub fn get_first_hit_color(&self, ray: &[f32; 3]) -> Option<[u8; 3]>{//RGBA, closer AABB is the first half, because it "partitiones" it with [<,=,>]

    if hit_box(ray, self){
      
      if self.childs.is_some(){//if AABB has childs test them first
        
        let inner = self.childs.as_ref().unwrap().0.get_first_hit_color(ray);

        if inner.is_some(){
          return inner;
        }else {
          return self.childs.as_ref().unwrap().1.get_first_hit_color(ray);
        }

      }else {//AABB is leaf
        
        for t in &self.mesh{
          if self::hit_triangle(ray, t){
            return Some([0xFFu8; 3]);
          }
        }
        return Some([0xFFu8; 3]);
      }

    }
    Some([0xFFu8; 3])
  }

  const fn get_min_max(m: &Vec<Triangle>) -> ((f32, f32), (f32, f32)){
    //TODO
    ((0f32, 0f32), (0f32, 0f32))
  }

}