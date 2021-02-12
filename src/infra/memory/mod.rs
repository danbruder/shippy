use crate::domain::{App, AppRepo};
use crate::prelude::*;

pub struct MemoryAppRepo;

impl MemoryAppRepo {
    pub fn new() -> Self {
        MemoryAppRepo
    }
}

#[async_trait]
impl AppRepo for MemoryAppRepo {
    async fn insert(&self, input: App) -> Result<App> {
        Ok(input)
    }
    async fn list(&self) -> Result<Vec<App>> {
        Ok(vec![])
    }
}
