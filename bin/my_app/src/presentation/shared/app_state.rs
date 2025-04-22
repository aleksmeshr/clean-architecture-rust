use std::sync::Arc;

use crate::usecases::get_product_usecase::GetProductUseCase;

pub struct AppState {
    pub app_name: String,
    pub get_product_usecase: Arc<dyn GetProductUseCase>,
}
