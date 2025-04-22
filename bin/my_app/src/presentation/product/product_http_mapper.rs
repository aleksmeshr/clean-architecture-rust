use crate::presentation::product::product_dto::ProductPayload;

use domain::product::product::Product;

pub fn to_api(entity: Product) -> ProductPayload{
    ProductPayload {
        id: entity.id,
        name: entity.name,
    }
}

pub fn to_entity(_payload: ProductPayload) -> Product {
    panic!("not implemented");
}
