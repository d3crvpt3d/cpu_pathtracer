#![allow(unused)]
use std::{clone, fmt::Write, ops::{Add, Mul, Sub}};

use glam::{vec3, Vec3};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use serde::{Deserialize, Serialize};
use rayon::prelude::*;

pub type Vertex = [f32;3];
pub type Mesh = Vec<Triangle>;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Triangle{
	pub a: Vec3,
	pub b: Vec3,
	pub c: Vec3,
	pub normal: Vec3,
	pub reflectiveness: f32,
	pub color: [f32; 3],
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

pub fn from_ascii(data: String, color: [f32; 3], reflectiveness: f32)->Mesh{
	
	let mut vec: Vec<Triangle> = Vec::with_capacity(64);
	let iterator: Vec<&str> = data.par_split_whitespace().collect();
	
	let bar = ProgressBar::new(iterator.len() as u64);
	bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
	.unwrap()
	.with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
	.progress_chars("=>-"));
	
	let zerot = Triangle{a: Vec3::ZERO, b: Vec3::ZERO, c: Vec3::ZERO, normal: Vec3::ZERO, reflectiveness, color};
	let mut t: Triangle = zerot.clone();
	let mut v_it = 0;
	
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
	
	vec
}

//TODO: read binary stl files