use std::{fs::File, io::BufReader};

pub struct Object{
  polygons: Vec<((f32, f32, f32), (f32, f32, f32), (f32, f32, f32))>,//polygon buffer for object
}

impl Object{

  pub fn new(path: &str) -> Self{

    Object{
      polygons: read_to_polygon_vec(path),
    }
  
  }

}

fn read_to_polygon_vec(path: &str) -> Vec<((f32, f32, f32), (f32, f32, f32), (f32, f32, f32))>{
  
  let b_reader = File::open(path).expect("file doesnt exist");
  
  let out_vec = Vec::with_capacity(b_reader.re);
  
  //TODO read file to polygon instead of debug tetraeder
  vec![
    (( 1,  1, -1), (-1, -1, -1), (-1,  1,  1)),//ACF
    (( 1,  1, -1), (-1, -1, -1), ( 1, -1,  1)),//ACH
    (( 1,  1, -1), (-1,  1,  1), ( 1, -1,  1)),//AFH
    ((-1, -1, -1), (-1,  1,  1), ( 1, -1,  1)),//CFH
  ]

}

fn norm_batch_to_polygon(bufreader) -> ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32)){

} 