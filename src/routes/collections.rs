use crate::models::vector_manager::Collection;
use std::collections::HashMap;

async fn make_collection(collections: &mut HashMap<String, Collection>, collection_name : &str, collection_dimension:usize) -> String{
    if collections.contains_key(collection_name) {
        return format!("Collection {} already exists", collection_name);
    }
    collections.insert(collection_name.to_string(), Collection::new(collection_name, collection_dimension));
    format!("Collection {} created", collection_name)
}