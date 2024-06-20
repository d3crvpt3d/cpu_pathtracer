mod raycaster;
mod renderer;
mod object_handler;
mod bvh_tree;

fn main() {

  let args: Vec<String> = std::env::args().collect();

  let mesh = object_handler::read_to_polygon_vec(&args[1]);

}