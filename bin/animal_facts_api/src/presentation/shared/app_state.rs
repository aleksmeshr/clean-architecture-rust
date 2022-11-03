use animal_facts_repositories::{
    postgres::dog_facts_repository::DogFactsRepository,
    http::cat_facts_repository::CatFactsRepository,
};

pub struct AppState {
    pub app_name: String,
    pub cats_repository: CatFactsRepository,
    pub dogs_repository: DogFactsRepository,
}
