use crate::presentation::product::{
    product_dto::ProductPayload,
    product_http_mapper,
};

use crate::presentation::shared::{
    app_state::AppState,
    errors::ErrorResponse,
};

use crate::usecases::{
    get_product_usecase::GetProductUseCase,
},

use actix_web::{get, web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_product_by_id);
}

#[get("/product/{product_id}")]
async fn get_product_by_id(
    data: web::Data<AppState>,
    path_params: web::Path<(i32,)>,
) -> Result<HttpResponse, ErrorResponse> {
    let product_id: i32 = path_params.into_inner().0;
    let get_product_usecase = &data.get_product_usecase;
    let product = get_product_usecase.execute(product_id).await;
    
    product.map_err(ErrorResponse::map_io_error).map(|fact| {
        HttpResponse::Ok().json(
            fact
                .into_iter()
                .map(product_http_mapper::to_api),
        )
    })
}
