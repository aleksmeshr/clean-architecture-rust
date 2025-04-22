use super::product_schema::*;

#[derive(Queryable, QueryableByName)]
#[diesel(table_name = products)]
pub struct ProductDbDto {
    pub id: i32,
    pub name: String,
}
