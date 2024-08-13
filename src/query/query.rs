use super::select::SelectQuery;
use super::update::UpdateQuery;

pub enum Query {}

impl Query {
    pub fn select() -> SelectQuery {
        SelectQuery::new()
    }
    pub fn update() -> UpdateQuery {
        UpdateQuery::new()
    }
}
