#![allow(unused)]
use std::{clone, fmt::Write, ops::{Add, Mul, Sub}};

use glam::Vec3;
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
}

impl Add for Triangle{
	type Output = Triangle;
	
	fn add(self, rhs: Self) -> Self::Output {
		
		Triangle{
			a: self.a + rhs.a,
			b: self.b + rhs.c,
			c: self.c + rhs.c,
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
		}
	}	
}

pub fn from_ascii(data: String)->Mesh{
	
	let mut vec: Vec<Triangle> = Vec::with_capacity(64);
	let iterator: Vec<&str> = data.par_split_whitespace().collect();
	
	let bar = ProgressBar::new(iterator.len() as u64);
	bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
	.unwrap()
	.with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
	.progress_chars("=>-"));
	
	let zerot = Triangle{a: Vec3::ZERO, b: Vec3::ZERO, c: Vec3::ZERO};
	let mut t: Triangle = zerot.clone();
	let mut v_it = 0;
	
	for (it, &word) in iterator.iter().enumerate(){
		
		if word == "loop" {
			
			t.a = Vec3::from_array([iterator[it+2].parse::<f32>().unwrap(),
			iterator[it+3].parse::<f32>().unwrap(),
			iterator[it+4].parse::<f32>().unwrap()]);

			t.b = Vec3::from_array([iterator[it+6].parse::<f32>().unwrap(),
			iterator[it+7].parse::<f32>().unwrap(),
			iterator[it+8].parse::<f32>().unwrap()]);

			t.c = Vec3::from_array([iterator[it+10].parse::<f32>().unwrap(),
			iterator[it+11].parse::<f32>().unwrap(),
			iterator[it+12].parse::<f32>().unwrap()]);
		
			vec.push(t);
			t = zerot.clone();
		}
		bar.inc(1);//added
	}
	bar.finish();//added
	
	vec
}