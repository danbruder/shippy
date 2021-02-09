use super::Kind;
use crate::prelude::*;

pub struct Id(Kind, String);

impl Id {
    pub fn new(kind: Kind) -> Self {
        let id = Uuid::new_v4().to_string();
        Id(kind, id)
    }
}
