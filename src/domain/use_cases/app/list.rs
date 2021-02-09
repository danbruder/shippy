//! List an App Usecase
use crate::prelude::*;

use crate::domain::entities::*;
use crate::domain::repo::*;

pub struct ListAppUsecase<A: AppRepo> {
    apps: A,
}

impl<A: AppRepo> ListAppUsecase<A> {
    pub async fn run(&self) -> Result<Vec<App>> {
        self.apps.list().await
    }
}
