//! Create an App Usecase
use crate::prelude::*;

use crate::domain::entities::*;
use crate::domain::repo::*;

pub struct CreateAppUsecase<A: AppRepo> {
    apps: A,
}

struct CreateAppInput {
    name: String,
}

impl<A: AppRepo> CreateAppUsecase<A> {
    pub async fn run(&self, input: CreateAppInput) -> Result<App> {
        let app = App::new(input.name);
        let app = self.apps.insert(app).await?;
        Ok(app)
    }
}
