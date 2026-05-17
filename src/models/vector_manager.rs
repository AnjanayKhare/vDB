use crate::models::utils::Logger;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VecRequest{
    pub collection: String,
    pub data: Vec<f32>
}

const LOGGER: Logger = Logger::new("VectorManager");

pub(crate) struct EmbeddingVector {
    data: Vec<f32> , // actual vector
    dimension :usize , // vector dimension
    l1_norm: f32
}



#[derive(Debug, Deserialize)]
pub struct MakeCollectionsRequest{
    pub dimension : usize,
    pub name : String
}

pub struct Collection{
    dimension: usize,
    pub name: String,
    data: Vec<EmbeddingVector>,
    length: usize,
}

impl Collection{
    pub fn get_dimension(&self) -> usize{
        self.dimension
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn new(name:&str, dimension:usize) -> Collection{
        Collection{
            dimension,
            name:name.to_string(),
            data: vec![],
            length: 0,
        }
    }
    
    pub fn add_embedding(&mut self, embedding:EmbeddingVector) {
        if embedding.dimension != self.dimension {
            LOGGER.error(&format!("Unable to add Embeddings to {} as the dimension of the not matching", self.name));
        }
        self.data.push(embedding);
        LOGGER.info(&format!("Added embedding vector \"{}\" at dimension {}", self.name, self.dimension));
        self.length += 1;
    }
}


impl EmbeddingVector {
    pub fn new(data:Vec<f32>) -> Self {
        let dimension = data.len();
        let l1_norm = Self::norm(&data) as f32;
        EmbeddingVector {
            data,
            dimension,
            l1_norm
        }
    }

    pub fn norm(data: &Vec<f32>) -> f32 {
        let square_sum = data.iter().fold(0.0, |sum,&x| sum + x*x as f32);
        square_sum / data.len() as f32
    }

}