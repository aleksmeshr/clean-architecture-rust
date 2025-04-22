use async_trait::async_trait;
use mockall::automock;

use crate::utils::error_handling_utils;
use domain::comment::comment_client::CommentClient;
use domain::product::{product::Product, product_repository::ProductRepository};

#[automock]
#[async_trait(?Send)]
pub trait GetProductUseCase {
    fn execute(&self) -> Result<Product, ApiError>;
}

pub struct GetProductUseCaseImpl<'a> {
    product_repository: &'a dyn ProductRepository,
    comment_client: &'a dyn CommentClient,
}

impl<'a> GetProductUseCaseImpl<'a> {
    pub fn new(
        product_repository: &'a dyn ProductRepository,
        comment_client: &'a dyn CommentClient,
    ) -> Self {
        GetProductUseCaseImpl { product_repository, comment_client }
    }
}

impl<'a> GetProductUseCase for GetProductUseCaseImpl<'a> {
    fn execute(&self, product_id: i32) -> Result<Product, ApiError> {
        let product = async { self.product_repository.find_by_id(product_id).await };

        match product {
            Ok(payload) => Ok(payload),
            Err(e) => {
                Err(error_handling_utils::application_error("Cannot query the product", Some(e)))
            }
        }
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};
    use animal_facts_ports::cat_facts::MockCatFacts;

    #[actix_rt::test]
    async fn test_should_return_list() {
        // given the "all cat facts" usecase repo returning a list of 2 entities
        let mut cat_fact_repository = MockCatFacts::new();
        cat_fact_repository.expect_get_all_cat_facts().with().times(1).returning(|| {
            Ok(vec![
                CatFactEntity { fact_txt: String::from("fact1"), fact_length: 1 },
                CatFactEntity { fact_txt: String::from("fact2"), fact_length: 2 },
            ])
        });

        // when calling usecase
        let get_all_cat_facts_usecase = GetAllCatFactsUseCase::new(&cat_fact_repository);
        let data = get_all_cat_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 2);
    }

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        // given the "all cat facts" usecase repo with an unexpected error
        let mut cat_fact_repository = MockCatFacts::new();
        cat_fact_repository
            .expect_get_all_cat_facts()
            .with()
            .times(1)
            .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let get_all_cat_facts_usecase = GetAllCatFactsUseCase::new(&cat_fact_repository);
        let data = get_all_cat_facts_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get all cat facts", result.message);
    }
}*/
