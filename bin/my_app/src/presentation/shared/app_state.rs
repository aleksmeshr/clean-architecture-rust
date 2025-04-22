use infra::{comment::http::comment_http_client, product::postgres::product_repository};

pub struct AppState {
    pub app_name: String,
    pub product_repository: ProductDbRepository,
    pub comment_client: CommentHttpClient,
    pub get_product_usecase: GetProductUseCase,
}
