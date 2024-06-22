pub mod ray_caster{
  use std::f32::consts::PI;

  //y z
  //↑↗
  //0→x
  pub fn get_rays<const X: usize, const Y: usize>(fov: usize, cam_pos: [f32; 3]) -> Vec<Vec<[f32; 3]>>{

    let a: f32 = f32::tan((fov/360) as f32 * PI);//a = steigung

    let mut yvec: Vec<Vec<[f32; 3]>> = Vec::with_capacity(Y);

    for y in 0..Y{

      let mut xvec: Vec<[f32; 3]> = Vec::with_capacity(X);

      for x in 0..X{
        
        let x_calc: f32 = a * ((2*x) as f32 / (X-1) as f32) - 1.;
        let y_calc: f32 = a * ((2*y) as f32 / (Y-1) as f32) - 1.;

        let ray: [f32; 3] = [cam_pos[0] + x_calc, cam_pos[1] + y_calc, cam_pos[2] + 1.];//cam_pos + XYZ
        xvec.push(ray);
      
      }

      yvec.push(xvec);
    }

    yvec
    //TODO: use polar coordinates
    //TODO: cast evenly rays from 'cam_pos' to 0-fov/2 and 0+fov/2
  }
}