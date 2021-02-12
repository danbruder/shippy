//! List an App Usecase
use crate::prelude::*;

use crate::domain::entities::*;
use crate::domain::repo::*;

pub struct ListAppsUsecase<A: AppRepo> {
    apps: A,
}

impl<A: AppRepo> ListAppsUsecase<A> {
    pub async fn run(&self) -> Result<Vec<App>> {
        self.apps.list().await
    }
}
