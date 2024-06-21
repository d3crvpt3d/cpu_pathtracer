pub mod ray_caster{

  pub fn get_rays<const X: usize, const Y: usize>(fov: usize, cam_pos: (f32, f32)) -> Vec<Vec<(f32, f32, f32)>>{


    //TODO: cast evenly rays from 'cam_pos' to 0-fov/2 and 0+fov/2




    vec![vec![(0f32, 0f32, 0f32); X]; Y]//DEBUG
  }
}