use crate::http::models::CatFactApiModel;
use animal_facts_domain::cat_fact_entity::CatFactEntity;

pub struct CatFactHttpMapper {}

impl CatFactHttpMapper {
    pub fn to_http(entity: CatFactEntity) -> CatFactApiModel {
        CatFactApiModel { fact: entity.fact_txt, length: entity.fact_length }
    }

    pub fn to_entity(http_obj: CatFactApiModel) -> CatFactEntity {
        CatFactEntity { fact_txt: http_obj.fact, fact_length: http_obj.length }
    }
}
