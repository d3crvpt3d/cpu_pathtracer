use std::fmt::Write;

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use serde::{Deserialize, Serialize};

//use stl_parser::{Mesh, Triangle, Vertex}; copied from this to use with serde
pub type Vertex = [f32;3];
pub type Line = (Vertex, Vertex);
pub type Slice = Vec<Vertex>;
///Coordinate helper
#[derive(Debug, Clone, Serialize, Deserialize )]
pub enum Axis
{
    X,
    Y,
    Z
}
///Triangle class. Mesh is list of these.
#[derive(Debug, Clone, Serialize, Deserialize )]
pub struct Triangle{
    pub vertices: [Vertex;3],
    pub lines: [Line;3]
}
///Mesh. This will be generated from .stl
#[derive(Debug, Clone, Serialize, Deserialize )]
pub struct Mesh
{
    pub triangles: Vec<Triangle>
}
impl Mesh
{
    ///Produces empty mesh
    pub fn new()->Mesh
    {
        Mesh{triangles: Vec::new()}
    }
    ///Give files text, not name
    pub fn from_ascii(data: String)->Mesh
    {
        let mut vec: Vec<Triangle> = Vec::new();
        let iterator: Vec<&str> =  data.split_whitespace().collect();

        //added
        let bar = ProgressBar::new(iterator.len() as u64);
        bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("=>-"));
        //added

        let mut vertex: Vec<Vertex> = Vec::new();
        let mut iter = 0;
        for word in iterator.clone(){
            if word == "vertex"
            {
                vertex.push([iterator.clone()[iter+1].parse::<f32>().unwrap(), iterator.clone()[iter+2].parse::<f32>().unwrap(), 
                            iterator.clone()[iter+3].parse::<f32>().unwrap()]);
            }
            if word == "endloop"
            {
                vec.push(Triangle::new([vertex.remove(0), vertex.remove(0), vertex.remove(0)]));
                vertex = Vec::new();
            }
            iter+=1;
            bar.inc(1);//added
        }
        bar.finish();//added
        Mesh{triangles: vec}
    }
    pub fn slice(&self, axis: Axis, point: f32)->Slice
    {
        let mut slice: Slice = Vec::new();
        match axis{
        Axis::X => {for triangle in &self.triangles{slice.append(&mut triangle.intersects_x(point))}}
        Axis::Y => {for triangle in &self.triangles{slice.append(&mut triangle.intersects_y(point))}}
        Axis::Z => {for triangle in &self.triangles{slice.append(&mut triangle.intersects_z(point))}}
        }
        return slice;
    }
}
impl Triangle
{
    ///Proper and easy way to make triangle.
    pub fn new(vertices: [Vertex;3])->Triangle
    {
        let line_a = (vertices[0], vertices[1]);
        let line_b = (vertices[1], vertices[2]);
        let line_c = (vertices[2], vertices[0]);
        Triangle{vertices: vertices, lines: [line_a, line_b, line_c]}
    }
    ///Gives points in triangles lines when x is known
    pub fn intersects_x(&self, x: f32)->Vec<Vertex>
    {
        let mut vec: Vec<Vertex> = Vec::new();
        for line in &self.lines
        {
            if line.0[2] < x && line.1[2] > x || line.0[2] > x && line.1[2] < x
            {
                
                let mut z = x*((line.1[2]-line.0[2])/(line.1[0]-line.0[0]));
                let mut y = z*((line.1[1]-line.0[1])/(line.1[0]-line.0[0]));
                if line.1[0]-line.0[0] == 0.0
                {
                    z = 0.0;
                    y = 0.0;
                }
                vec.push([x, line.0[1]+y, line.0[2]+z]);
            }
        }
         return vec;
    }
    ///Gives points in triangles lines when y is known
    pub fn intersects_y(&self, y: f32)->Vec<Vertex>
    {
        let mut vec: Vec<Vertex> = Vec::new();
        for line in &self.lines
        {
            if line.0[2] < y && line.1[2] > y || line.0[2] > y && line.1[2] < y
            {
                let mut x = y*((line.1[0]-line.0[0])/(line.1[1]-line.0[1]));
                let mut z = x*((line.1[2]-line.0[2])/(line.1[0]-line.0[0]));
                if line.1[1]-line.0[1] == 0.0
                {
                    x = 0.0;
                    z = 0.0;
                }
                vec.push([line.0[0]+x, y, line.0[2]+z]);
            }
        }
         return vec;
    }
    ///Gives points in triangles lines when z is known
    pub fn intersects_z(&self, z: f32)->Vec<Vertex>
    {
        let mut vec: Vec<Vertex> = Vec::new();
        for line in &self.lines
        {
            if line.0[2] < z && line.1[2] > z || line.0[2] > z && line.1[2] < z
            {
                let mut x = z*((line.1[0]-line.0[0])/(line.1[2]-line.0[2]));
                let mut y = x*((line.1[1]-line.0[1])/(line.1[0]-line.0[0]));
                if line.1[2]-line.0[2] == 0.0
                {
                    x = 0.0;
                    y = 0.0;
                }
                vec.push([line.0[0]+x, line.0[1]+y, z]);
            }
        }
         return vec;
    }
}