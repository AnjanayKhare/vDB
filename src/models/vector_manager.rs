use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VecRequest{
    pub collection: String,
    pub data: Vec<i32>
}

struct VectorManager {
    data: Vec<i32> , // actual vector
    dimension :usize , // vector dimension
    l1_norm: f32
}


pub struct MakeCollectionsRequest{
    dimension : usize,
    name : String
}
pub struct Collection{
    dimension: usize,
    name: String,
}

impl Collection{
    pub fn new(name:&str, dimension:usize) -> Collection{
        Collection{
            dimension,
            name:name.to_string()
        }
    }
}


impl VectorManager {
    pub fn new(data:Vec<i32>) -> Self {
        let dimension = data.len();
        let l1_norm = Self::norm(&data) as f32;
        VectorManager {
            data,
            dimension,
            l1_norm
        }
    }

    pub fn norm(data: &Vec<i32>) -> f32 {
        let square_sum = data.iter().fold(0.0, |sum,&x| sum + x as f32);
        square_sum / data.len() as f32
    }

}