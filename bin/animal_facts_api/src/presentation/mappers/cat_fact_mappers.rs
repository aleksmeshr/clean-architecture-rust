use crate::presentation::models::cat_fact_models::CatFactPayload;
use animal_facts_domain::cat_fact_entity::CatFactEntity;

pub fn to_api(entity: CatFactEntity) -> CatFactPayload{
    CatFactPayload {
        fact: entity.fact_txt,
        nb_chars: entity.fact_length,
    }
}

pub fn to_entity(_payload: CatFactPayload) -> CatFactEntity {
    panic!("not implemented");
}
