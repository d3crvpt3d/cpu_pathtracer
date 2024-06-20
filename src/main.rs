use bvh_tree::BvhTree;
use renderer::ImgBuffer;
use raycaster::ray_caster::get_rays;

mod raycaster;
mod renderer;
mod object_handler;
mod bvh_tree;

fn main() {

  let fov: usize = 90;
  let camera_pos: (f32, f32) = (0f32, 0f32);
  const PIXELS: (usize, usize) = (160, 90);



  let args: Vec<String> = std::env::args().collect();

  let mesh = object_handler::stl_to_vec(&args[1]);

  let bvh = BvhTree::from_mesh(mesh);

  let mut img_buffer: ImgBuffer<{PIXELS.0}, {PIXELS.1}> = ImgBuffer::new();

  let rays: [(f32, f32, f32); PIXELS.0 * PIXELS.1] = get_rays(fov, camera_pos);


  //check if ray intersects with a polygon
  rays.iter().enumerate().for_each(|(px, ray)| {

    let curr_x = px % PIXELS.1;
    let curr_y = px - curr_x * PIXELS.0;

    let color:(u16, u16, u16, u16)  = bvh.get_first_hit_color(*ray);

    //128*100/256 = 128*10 >> 8 = 1280 >> 8 = 5;
    img_buffer.array[curr_x][curr_y] = (((color.0 * color.3) >> 8) as u8, ((color.0 * color.3) >> 8) as u8, ((color.0 * color.3) >> 8) as u8);
  });
}