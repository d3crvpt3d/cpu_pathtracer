pub mod ray_caster{
  use std::f32::consts::PI;

  //y z
  //↑↗
  //0→x
  pub fn get_rays<const X: usize, const Y: usize>(fov: usize) -> Vec<Vec<[f32; 3]>>{

    let a: f32 = f32::tan((fov as f32 / 360f32) * PI);//a = steigung
    let xf = (X-1) as f32;
    let yf = (X-1) as f32;

    let mut yvec: Vec<Vec<[f32; 3]>> = Vec::with_capacity(Y);

    for y in (Y-1)..=0{//flip y-axis for image::

      let mut xvec: Vec<[f32; 3]> = Vec::with_capacity(X);

      for x in 0..X{
        let x_calc: f32 = a * (((2*x) as f32 / xf) - 1.);
        let y_calc: f32 = a * (((2*y) as f32 / yf) - 1.);

        xvec.push([x_calc, y_calc, 1.]);
      }

      yvec.push(xvec);
    }

    //DEBUG
    //DEBUG
    //DEBUG
    let mut rays_img = image::RgbImage::new(X as u32, Y as u32);
    
    for (x, y, pixel) in rays_img.enumerate_pixels_mut(){
      *pixel = image::Rgb([ (yvec[y as usize][x as usize][0].atan() * 255f32) as u8,
                            (yvec[y as usize][x as usize][1].atan() * 255f32) as u8,
                            (yvec[y as usize][x as usize][2].atan() * 255f32) as u8]
                          );
    }
    rays_img.save_with_format("rays_image.png", image::ImageFormat::Png).unwrap();
    //DEBUG
    //DEBUG
    //DEBUG

    yvec
    //TODO: use polar coordinates
  }

  pub fn transform_direction(rays: Vec<Vec<[f32; 3]>>) -> Vec<Vec<[f32; 3]>>{
    //TODO
    rays
  }
}