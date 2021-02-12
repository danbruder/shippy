//! Create an App Usecase
use crate::prelude::*;

use crate::domain::entities::*;
use crate::domain::repo::*;

pub struct CreateAppUseCase<A: AppRepo> {
    apps: A,
}

pub struct CreateAppInput {
    pub name: String,
}

impl<A: AppRepo> CreateAppUseCase<A> {
    pub fn new(apps: A) -> Self {
        Self { apps }
    }

    pub async fn run(&self, input: CreateAppInput) -> Result<App> {
        let app = App::new(input.name);
        let app = self.apps.insert(app).await?;
        Ok(app)
    }
}
