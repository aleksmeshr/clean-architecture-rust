use crate::product::postgres::product_db_dto::ProductDbDto;
use domain::product::product::Product;

pub struct ProductDbMapper {}

impl ProductDbMapper {
    pub fn to_db(entity: Product) -> ProductDbDto {
        ProductDbDto { id: entity.id, name: entity.name }
    }

    pub fn to_entity(model: ProductDbDto) -> Product {
        Product { id: model.id, name: model.name }
    }
}
