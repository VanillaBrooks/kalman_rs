extern crate nalgebra as na;
use na::{Point3, Matrix4 as Matrix, Vector3 as Vector};
use super::Traits;

pub struct Trapezoid{
    points : [na::Point3<f32>; 4],
    tfm: na::Affine3<f32>
}

impl Trapezoid{
    fn new(points: [Point3<f32>; 4], tfm_matrix: Matrix<f32>) -> Result<Trapezoid, &'static str>{
        
        let affine_transform = na::Affine3::from_matrix_unchecked(tfm_matrix);

        match affine_transform.try_inverse(){
            Some(x) => return Ok(Trapezoid{points: points, tfm: affine_transform}),
            None => return Err("matrix was not invertable")

        }
    }
}




impl Traits::Transform for Trapezoid{
    fn to_global(&self, input_point: Point3<f32>)-> na::Point3<f32>{
        return self.tfm * input_point;
    }
    
    fn to_local(&self, input_point: Point3<f32>) -> na::Point3<f32>{
        self.tfm.inverse() * input_point
    }

    fn contains_from_local(&self, input: &Point3<f32>) ->bool{
        Traits::quadralateral_contains(&self.points, &input)
    }
}