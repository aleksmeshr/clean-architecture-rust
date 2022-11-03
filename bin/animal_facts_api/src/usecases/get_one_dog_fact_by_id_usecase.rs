use animal_facts_domain::{
    dog_fact_entity::DogFactEntity,
    error::ApiError,
};
use animal_facts_ports::dog_facts::DogFacts;
use crate::utils::error_handling_utils;

pub struct GetOneDogFactByIdUseCase<'a> {
    dog_fact_id: &'a i32,
    repository: &'a dyn DogFacts,
}

impl<'a> GetOneDogFactByIdUseCase<'a> {
    pub fn new(dog_fact_id: &'a i32, repository: &'a dyn DogFacts) -> Self {
        GetOneDogFactByIdUseCase { dog_fact_id, repository }
    }
}

impl<'a> GetOneDogFactByIdUseCase<'a> {
    pub async fn execute(&self) -> Result<DogFactEntity, ApiError> {
        let dog_fact = self.repository.get_dog_fact_by_id(*self.dog_fact_id).await;

        match dog_fact {
            Ok(dog_fact) => Ok(dog_fact),
            Err(e) => {
                Err(error_handling_utils::application_error("Cannot get single dog fact", Some(e)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use std::io::{Error, ErrorKind};

    use animal_facts_ports::dog_facts::MockDogFacts;

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all dog facts" usecase repo with an unexpected random error
        let mut dog_fact_repository = MockDogFacts::new();
        dog_fact_repository
            .expect_get_dog_fact_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let get_one_dog_fact_by_id_usecase =
            GetOneDogFactByIdUseCase::new(&1, &dog_fact_repository);
        let data = get_one_dog_fact_by_id_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get single dog fact", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one dog fact by id" usecase repo returning one result
        let mut dog_fact_repository = MockDogFacts::new();
        dog_fact_repository
            .expect_get_dog_fact_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| Ok(DogFactEntity { fact_id: 1, fact: String::from("fact1") }));

        // when calling usecase
        let get_one_dog_fact_by_id_usecase =
            GetOneDogFactByIdUseCase::new(&1, &dog_fact_repository);
        let data = get_one_dog_fact_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.fact_id, 1);
        assert_eq!(data.fact, "fact1");
    }
}
