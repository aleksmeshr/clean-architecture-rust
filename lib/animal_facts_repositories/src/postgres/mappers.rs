use crate::postgres::models::DogFact;
use animal_facts_domain::dog_fact_entity::DogFactEntity;

pub struct DogFactDbMapper {}

impl DogFactDbMapper {
    pub fn to_db(entity: DogFactEntity) -> DogFact {
        DogFact { id: entity.fact_id, fact: entity.fact }
    }

    pub fn to_entity(model: DogFact) -> DogFactEntity {
        DogFactEntity { fact_id: model.id, fact: model.fact }
    }
}
