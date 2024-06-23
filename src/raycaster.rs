pub mod ray_caster{
  use std::f32::consts::PI;

  //y z
  //↑↗
  //0→x
  pub fn get_rays<const X: usize, const Y: usize>(fov: usize, cam_pos: [f32; 3]) -> Vec<Vec<[f32; 3]>>{//TODO:fix

    let a: f32 = f32::tan((fov as f32/360f32) * PI);//a = steigung

    let mut yvec: Vec<Vec<[f32; 3]>> = Vec::with_capacity(Y);

    for y in 0..Y{

      let mut xvec: Vec<[f32; 3]> = Vec::with_capacity(X);

      for x in 0..X{
        let x_calc: f32 = a * ((2*x) as f32 / (X-1) as f32) - 1.;
        let y_calc: f32 = a * ((2*y) as f32 / (Y-1) as f32) - 1.;

        xvec.push([x_calc, y_calc, 1.]);
      }

      yvec.push(xvec);
    }

    yvec
    //TODO: use polar coordinates
  }

  pub fn transform_direction(rays: Vec<Vec<[f32; 3]>>) -> Vec<Vec<[f32; 3]>>{
    //TODO
    rays
  }
}