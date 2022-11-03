use crate::presentation::models::dog_fact_models::DogFactPayload;
use animal_facts_domain::dog_fact_entity::DogFactEntity; 

pub fn to_api(entity: DogFactEntity) -> DogFactPayload {
    DogFactPayload { fact_id: entity.fact_id, txt: entity.fact }
}

pub fn to_entity(_payload: DogFactPayload) -> DogFactEntity {
    panic!("not implemented");
}
