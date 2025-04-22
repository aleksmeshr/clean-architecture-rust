use crate::product::postgres::product_schema::*;

#[derive(Queryable, QueryableByName)]
#[table_name = "products"]
pub struct ProductDbDto {
    pub id: i32,
    pub name: String,
}
