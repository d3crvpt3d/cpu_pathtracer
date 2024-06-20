pub mod ray_caster{

  pub fn get_rays<const RAY_NUMBER: usize>(fov: usize, cam_pos: (f32, f32)) -> [(f32, f32, f32); RAY_NUMBER]{


    //TODO: cast evenly rays from 'cam_pos' to 0-fov/2 and 0+fov/2


    [(0f32, 0f32, 0f32); RAY_NUMBER]//DEBUG
  }
}