#[cfg(test)]
use ray_caster::get_rays;

pub mod ray_caster{
  use std::f32::consts::PI;

  //y z
  //↑↗
  //0→x
  pub fn get_rays(fov: usize, pixels: (usize, usize), subdivisions: u32) -> Vec<Vec<[f32; 3]>>{

    let new_pix = (pixels.0 * subdivisions as usize, pixels.1 * subdivisions as usize);

    //a = steigung
    let ax: f32 = f32::tan((fov as f32 / 360f32) * PI);//a/X*Y/1
    let ay: f32 = ax*((new_pix.1-1) as f32/(new_pix.0-1) as f32);//a/X*Y/1


    let xf = (new_pix.0 as usize-1) as f32;
    let yf = (new_pix.1 as usize-1) as f32;

    let mut yvec: Vec<Vec<[f32; 3]>> = Vec::with_capacity(new_pix.1);


    let mut y = new_pix.1-1;

    loop{

      let mut xvec: Vec<[f32; 3]> = Vec::with_capacity(new_pix.0);

      for x in 0..new_pix.0{
        let x_calc: f32 = ax * (((2*x) as f32 / xf) - 1.);
        let y_calc: f32 = ay * (((2*y) as f32 / yf) - 1.);

        xvec.push([x_calc, y_calc, 1.]);
      }

      yvec.push(xvec);
      
      if y == 0{
        break;
      }
      y -= 1
    }

    //DEBUG
    //DEBUG
    //DEBUG
    // let mut rays_img = image::RgbImage::new(X as u32, pixels.0 as u32);
    
    // for (x, y, pixel) in rays_img.enumerate_pixels_mut(){
    //   *pixel = image::Rgb([ (yvec[y as usize][x as usize][0].atan() * 255f32) as u8,
    //                         (yvec[y as usize][x as usize][1].atan() * 255f32) as u8,
    //                         (yvec[y as usize][x as usize][2].atan() * 255f32) as u8]
    //                       );
    // }
    // rays_img.save_with_format("storage/rays_image.png", image::ImageFormat::Png).unwrap();
    //DEBUG
    //DEBUG
    //DEBUG

    yvec
  }

  pub fn transform_direction(rays: Vec<Vec<[f32; 3]>>, _subdivisions: u32, _xyz: (f32, f32, f32)) -> Vec<Vec<[f32; 3]>>{
    //TODO
    rays
  }
}

#[test]
fn test(){
  let rays = get_rays(90, (4, 2), 0);

  let mut comp: Vec<Vec<[f32; 3]>> = Vec::new();
  
  //x_fov = 90, y_fov = 45
  comp.push(vec![ [-1.        , 0.33333334, 1.],
                  [-0.3333333 , 0.33333334, 1.],
                  [ 0.33333337, 0.33333334, 1.],
                  [ 1.        , 0.33333334, 1.]]
  );//y-max
  comp.push(vec![ [-1.        ,-0.33333334, 1.],
                  [-0.3333333 ,-0.33333334, 1.],
                  [ 0.33333337,-0.33333334, 1.],
                  [ 1.        ,-0.33333334, 1.]]);//y-min

  assert_eq!(rays, comp);
}
