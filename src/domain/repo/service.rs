//! Service Repo

use crate::domain::entities::*;
use crate::prelude::*;

#[async_trait]
pub trait ServiceRepo {
    async fn insert(&self, input: Service) -> Result<Service>;
}
