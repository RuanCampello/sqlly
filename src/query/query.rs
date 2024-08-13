use super::select::SelectQuery;
use super::update::UpdateQuery;

pub enum Query {
    Select(SelectQuery),
    Update(UpdateQuery),
}

impl Query {
    pub fn select() -> SelectQuery {
        SelectQuery::new()
    }
    pub fn update() -> UpdateQuery {
        UpdateQuery::new()
    }
}
