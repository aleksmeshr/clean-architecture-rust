use crate::product::postgres::product_db_dto::ProductDbDto;
use domain::product::Product;

pub struct ProductDbMapper {}

impl ProductDbMapper {
    pub fn to_db(entity: Product) -> ProductDbDto {
        DogFact { id: entity.id, fact: entity.name }
    }

    pub fn to_entity(model: ProductDbDto) -> Product {
        SomeEntity { id: model.id, name: model.fact }
    }
}
