//! App Repo

use crate::domain::entities::*;
use crate::prelude::*;

#[async_trait]
pub trait AppRepo {
    async fn insert(&self, input: App) -> Result<App>;
    async fn list(&self) -> Result<Vec<App>>;
}
