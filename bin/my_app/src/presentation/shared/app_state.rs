use infra::{
    product::postgres::product_repository,
    comment::http::comment_http_client,
};

pub struct AppState {
    pub app_name: String,
    pub product_repository: ProductDbRepository,
    pub comment_client: CommentHttpClient,
    pub get_product_usecase: GetProductUseCase,
}
