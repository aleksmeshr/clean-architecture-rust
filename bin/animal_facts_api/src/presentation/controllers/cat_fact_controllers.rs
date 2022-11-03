use crate::{
    presentation::{
        mappers::cat_fact_mappers,
        models::cat_fact_models::CatFactPayload,
        shared::{
            app_state::AppState,
            errors::ErrorResponse,
        },
    },
    usecases::{
        get_all_cat_facts_usecase::GetAllCatFactsUseCase,
        get_one_random_cat_fact_usecase::GetOneRandomCatFactUseCase,
    },
};
use actix_web::{get, web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_cat_facts).service(get_one_random_cat_fact);
}

#[get("/")]
async fn get_all_cat_facts(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_all_cat_facts_usecase = GetAllCatFactsUseCase::new(&data.cats_repository);
    let cat_facts = get_all_cat_facts_usecase.execute().await;

    cat_facts.map_err(ErrorResponse::map_io_error).map(|facts| {
        HttpResponse::Ok().json(
            facts
                .into_iter()
                .map(cat_fact_mappers::to_api)
                .collect::<Vec<CatFactPayload>>(),
        )
    })
}

#[get("/random")]
async fn get_one_random_cat_fact(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_one_random_cat_fact_usecase = GetOneRandomCatFactUseCase::new(&data.cats_repository);
    let cat_fact = get_one_random_cat_fact_usecase.execute().await;

    cat_fact
        .map_err(ErrorResponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(cat_fact_mappers::to_api(fact)))
}
