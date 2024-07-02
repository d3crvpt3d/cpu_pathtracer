//#![allow(unused)]
use std::{fmt::Write, fs::File, io::{self, BufReader, Read}, ops::{Add, Mul, Sub}};

use glam::{vec3, Vec3};
use indicatif::{ProgressBar, ProgressIterator, ProgressState, ProgressStyle};
use rayon::prelude::*;

pub type Mesh = Vec<Triangle>;

#[derive(Clone, Copy, Default)]
#[derive(Debug)]
pub struct Triangle{
	pub a: Vec3,
	pub b: Vec3,
	pub c: Vec3,
	pub normal: Vec3,
	pub reflectiveness: f32,
	pub color: [f32; 3],
}

//efficiently create triangle from le_bytes //https://stackoverflow.com/questions/76749778/what-is-the-most-idiomatic-way-to-convert-a-slice-of-u8-array-into-u32-using-u32
impl From<([u8; 50], [u8; 3], f32)> for Triangle{

	fn from(val: ([u8; 50], [u8; 3], f32)) -> Self{
		Triangle{ 
			normal: Vec3{
				x: f32::from_le_bytes(<[u8; 4]>::try_from(&val.0[0..4]).unwrap()),
				y: f32::from_le_bytes(<[u8; 4]>::try_from(&val.0[4..8]).unwrap()),
				z: f32::from_le_bytes(<[u8; 4]>::try_from(&val.0[8..12]).unwrap())},

			a: Vec3{
				x: f32::from_le_bytes(<[u8; 4]>::try_from(&val.0[12..16]).unwrap()),
				y: f32::from_le_bytes(<[u8; 4]>::try_from(&val.0[16..20]).unwrap()),
				z: f32::from_le_bytes(<[u8; 4]>::try_from(&val.0[20..24]).unwrap())},

			b: Vec3{
				x: f32::from_le_bytes(<[u8; 4]>::try_from(&val.0[24..28]).unwrap()),
				y: f32::from_le_bytes(<[u8; 4]>::try_from(&val.0[28..32]).unwrap()),
				z: f32::from_le_bytes(<[u8; 4]>::try_from(&val.0[32..36]).unwrap())},

			c: Vec3{
				x: f32::from_le_bytes(<[u8; 4]>::try_from(&val.0[36..40]).unwrap()),
				y: f32::from_le_bytes(<[u8; 4]>::try_from(&val.0[40..44]).unwrap()),
				z: f32::from_le_bytes(<[u8; 4]>::try_from(&val.0[44..48]).unwrap())},

			color: [val.1[0] as f32, val.1[1] as f32, val.1[2] as f32],
			reflectiveness: val.2,
		}
	}

}

impl Add for Triangle{
	type Output = Triangle;
	
	fn add(self, rhs: Self) -> Self::Output {
		Triangle{
			a: self.a + rhs.a,
			b: self.b + rhs.c,
			c: self.c + rhs.c,
			normal: self.normal,
			reflectiveness: 0f32,
			color: [0.; 3],
		}
	}
}

impl Sub for Triangle{
	type Output = Triangle;
	
	fn sub(self, rhs: Self) -> Self::Output {
		
		Triangle{
			a: self.a - rhs.a,
			b: self.b - rhs.c,
			c: self.c - rhs.c,
			normal: self.normal,
			reflectiveness: 0f32,
			color: [0.; 3],
		}
	}
}

impl Mul for Triangle {
	type Output = Triangle;

	fn mul(self, rhs: Self) -> Self::Output {
		
		Triangle{
			a: self.a * rhs.a,
			b: self.b * rhs.c,
			c: self.c * rhs.c,
			normal: self.normal,
			reflectiveness: 0f32,
			color: [0.; 3],
		}
	}	
}

//https://de.wikipedia.org/wiki/STL-Schnittstelle
pub fn from_binary(path: &str, color: [f32; 3], reflectiveness: f32) -> std::io::Result<Mesh>{

	//todo!("read right");

	let mut raw_bytes: Vec<u8> = Vec::with_capacity(File::open(path).unwrap().metadata().unwrap().len() as usize);

	BufReader::new(std::fs::File::open(path)?).read_to_end(&mut raw_bytes)?;

	let mut triangle_iter = raw_bytes[84..].chunks(50);

	//Progress Bar
	triangle_iter.by_ref().into_iter().progress_with_style(ProgressStyle::with_template(
		"{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
	)
	.unwrap()
	.with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
	.progress_chars("=>-"));
	//Progress Bar


	//color
	let mut this_color = [color[0] as u8, color[1] as u8, color[2] as u8];

	let s = String::from_utf8(raw_bytes[0..80].to_vec()).unwrap();

	let sides = s.split_once("COLOR=");

	if sides.is_some(){
		let rgb = sides.unwrap().1.split_at(6).0;

		this_color = [u8::from_str_radix(&rgb[0..2], 16).unwrap(),
									u8::from_str_radix(&rgb[2..4], 16).unwrap(),
									u8::from_str_radix(&rgb[4..6], 16).unwrap()];
		
		println!("Color: 0x{}", rgb);
	}
	//color


	//reflect
	let mut this_reflect = reflectiveness;

	let refl = s.split_once("REFLECT=");

	if refl.is_some(){
		this_reflect = refl.unwrap().1[0..32].parse::<f32>().unwrap();
	}
	//reflect

	let vec_length = triangle_iter.len();

	println!("Triangles: {}", vec_length);

	let mut vec: Vec<Triangle> = Vec::with_capacity(vec_length);
	
	//read triangles to vec and create Triangle-Vector

	let mut bytes_buffer: [u8; 50];//create byte buffer

	loop{
		
		let x = triangle_iter.next();

		if x.is_none(){
			break;
		}

		bytes_buffer = <[u8; 50]>::try_from(x.unwrap()).unwrap();

		vec.push(Triangle::from((bytes_buffer, this_color, this_reflect)));
		
	}
	

	Ok(vec)
}

pub fn from_ascii(path: &str, color: [f32; 3], reflectiveness: f32)-> io::Result<Mesh>{
	
	let mut vec: Vec<Triangle> = Vec::with_capacity(64);
	
	let mut data = String::new();
	
	BufReader::new(File::open(path)?).read_to_string(&mut data)?;

	let iterator: Vec<&str> = data.par_split_whitespace().collect();
	
	let bar = ProgressBar::new(iterator.len() as u64);
	bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
	.unwrap()
	.with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
	.progress_chars("=>-"));
	
	let zerot = Triangle{a: Vec3::ZERO, b: Vec3::ZERO, c: Vec3::ZERO, normal: Vec3::ZERO, reflectiveness, color};
	let mut t: Triangle = zerot.clone();
	
	for (it, &word) in iterator.iter().enumerate(){
		
		if word == "facet" {

			t.normal = vec3(iterator[it+2].parse::<f32>().unwrap(), iterator[it+3].parse::<f32>().unwrap(), iterator[it+4].parse::<f32>().unwrap()).normalize();

			t.a = vec3(iterator[it+8].parse::<f32>().unwrap(), iterator[it+9].parse::<f32>().unwrap(), iterator[it+10].parse::<f32>().unwrap());
			t.b = vec3(iterator[it+12].parse::<f32>().unwrap(), iterator[it+13].parse::<f32>().unwrap(), iterator[it+14].parse::<f32>().unwrap());
			t.c = vec3(iterator[it+16].parse::<f32>().unwrap(), iterator[it+17].parse::<f32>().unwrap(), iterator[it+18].parse::<f32>().unwrap());

			vec.push(t);
			t = zerot.clone();
		}
		bar.inc(1);//added
	}
	bar.finish();//added
	
	Ok(vec)
}

//TODO: read binary stl files