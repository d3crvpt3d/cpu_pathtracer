#![allow(unused)]
use stl_parser::{Mesh, Triangle};
//use serde::{Serialize, Deserialize};

//#[derive(Serialize, Deserialize)]
pub struct BvhTree{
  root: Box<Volume>,
  camera_pos: [f32; 3],
}

impl BvhTree{
  
  pub fn from_mesh(m: Mesh, max_elements: usize, camera_pos: [f32; 3]) -> Self{
    BvhTree{
      root: Box::new(Volume::new(m, max_elements)),
      camera_pos,
    }
  }

  pub fn get_first_hit_color(&self, ray: &[f32; 3]) -> Option<[u8; 3]>{//RGBA

    //todo
    Some([0x00u8; 3])
  }

}
#[allow(unused)]
//#[derive(Serialize, Deserialize)]
struct Volume{
  max_elements: usize,
  //#[serde(with = "MeshDef")]
  mesh: Mesh,
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

  pub fn new(m: Mesh, max_elements: usize) -> Self{
    let bounding_box:((f32, f32), (f32, f32)) = Volume::get_min_max(&m);
    Volume{
      max_elements,
      mesh: m.clone(),
      bounding_box,
      num_elements: m.triangles.len(),
      axis: 0,
      childs: None,
    }
  }

  //partition triangles and give [0..len/2] = child.0, [len/2..len-1] = child.1
  pub fn split(&mut self, max_elements: usize, axis: u8) -> Option<(Box<Volume>,Box<Volume>)>{

    let n = self.num_elements;
    
    let mut mesh_1 = Mesh::new();
    let mut mesh_2 = Mesh::new();

    let mut median: f32 = 0f32;

    self.mesh.triangles.iter().for_each(|e| {
      median += e.vertices[0][axis as usize];
    });

    //partition at n/2
    self.mesh.triangles.select_nth_unstable_by( n/2,|e1, e2| {
      e1.vertices[0][axis as usize].partial_cmp(&e2.vertices[0][axis as usize]).expect("some float is NaN")
    });

    mesh_1.triangles = self.mesh.triangles[0..n/2].to_vec();//[0..n/2]
    mesh_2.triangles = self.mesh.triangles[n/2..n].to_vec();//[n/2..n]

    let mut vol1 = Volume::new(mesh_1, max_elements);
    let mut vol2 = Volume::new(mesh_2, max_elements);


    let nxt_axis: u8 = (self.axis + 1) % 3;//increment axis for children

    //recursively split childs if elements > max_elements
    if vol1.mesh.triangles.len() > vol1.max_elements{
      vol1.split(max_elements, nxt_axis);
    }

    if vol2.mesh.triangles.len() > vol2.max_elements{
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

  const fn get_min_max(m: &Mesh) -> ((f32, f32), (f32, f32)){
    //TODO
    ((0f32, 0f32), (0f32, 0f32))
  }

}


// #[derive(Serialize, Deserialize)]
// #[serde(remote = "Triangle")]
// struct TriangleDef{
//   pub vertices: [[f32; 3]; 3],
//   pub lines: [([f32; 3], [f32; 3]); 3],
// }

// #[derive(Serialize, Deserialize)]
// #[serde(remote = "Mesh")]
// struct MeshDef{
//   #[serde(with = "TriangleDef")]
//   pub triangles: Vec<Triangle>,
// }