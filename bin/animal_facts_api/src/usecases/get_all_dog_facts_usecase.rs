use animal_facts_ports::dog_facts::DogFacts;
use animal_facts_domain::{
    dog_fact_entity::DogFactEntity,
    error::ApiError,
};
use crate::utils::error_handling_utils;

pub struct GetAllDogFactsUseCase<'a> {
    repository: &'a dyn DogFacts,
}

impl<'a> GetAllDogFactsUseCase<'a> {
    pub fn new(repository: &'a dyn DogFacts) -> Self {
        GetAllDogFactsUseCase { repository }
    }
}

impl<'a> GetAllDogFactsUseCase<'a> {
    pub async fn execute(&self) -> Result<Vec<DogFactEntity>, ApiError> {
        let dog_facts = self.repository.get_all_dog_facts().await;

        match dog_facts {
            Ok(facts) => Ok(facts),
            Err(e) => {
                Err(error_handling_utils::application_error("Cannot get all dog facts", Some(e)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use animal_facts_ports::dog_facts::MockDogFacts;

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all dog facts" usecase repo with an unexpected random error
        let mut dog_fact_repository = MockDogFacts::new();
        dog_fact_repository
            .expect_get_all_dog_facts()
            .with()
            .times(1)
            .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&dog_fact_repository);
        let data = get_all_dog_facts_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get all dog facts", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_empty_list() {
        // given the "all dog facts" usecase repo returning an empty list
        let mut dog_fact_repository = MockDogFacts::new();
        dog_fact_repository
            .expect_get_all_dog_facts()
            .with()
            .times(1)
            .returning(|| Ok(Vec::<DogFactEntity>::new()));

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&dog_fact_repository);
        let data = get_all_dog_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 0);
    }

    #[actix_rt::test]
    async fn test_should_return_list() {
        // given the "all dog facts" usecase repo returning a list of 2 entities
        let mut dog_fact_repository = MockDogFacts::new();
        dog_fact_repository.expect_get_all_dog_facts().with().times(1).returning(|| {
            Ok(vec![
                DogFactEntity { fact_id: 1, fact: String::from("fact1") },
                DogFactEntity { fact_id: 2, fact: String::from("fact2") },
            ])
        });

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&dog_fact_repository);
        let data = get_all_dog_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 2);
    }
}
